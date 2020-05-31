use crate::client_controller::ClientControllerProxy;
use mahjong::{
    anyhow::*, client::LocalState, hand::Call, match_state::*, messages::MatchEvent,
    strum::IntoEnumIterator, tile,
};
use rand::{seq::SliceRandom, SeedableRng};
use rand_pcg::*;
use std::collections::HashMap;
use thespian::{Actor, Remote};
use tile::{TileId, Wind};
use tracing::*;

#[derive(Debug, Actor)]
pub struct MatchController {
    rng: Pcg64Mcg,
    state: MatchState,

    /// Mapping of which client controls which player seat. Key is the index of the
    clients: HashMap<Wind, ClientProxy>,
    num_players: u8,

    remote: Remote<Self>,
}

impl MatchController {
    // TODO: Verify that the requested number of player is at least 1 and no more than 4.
    pub fn new(id: MatchId, num_players: u8, remote: Remote<Self>) -> Self {
        let mut rng = Pcg64Mcg::from_entropy();

        // Generate the tileset and shuffle it.
        let mut tiles = tile::TILE_SET.clone();
        tiles.shuffle(&mut rng);

        let mut state = MatchState::new(id, tiles);

        // For the east player, have them draw the tile for their first turn.
        state.draw_for_player(Wind::East).unwrap();

        Self {
            rng,
            state,
            clients: Default::default(),
            num_players,
            remote,
        }
    }

    fn broadcast(&mut self, event: MatchEvent) {
        trace!(
            ?event,
            "Broadcasting event to {} connected client(s)",
            self.clients.len()
        );

        for client in self.clients.values_mut() {
            client
                .send_event(event.clone())
                .expect("Disconnected from client controller");
        }
    }
}

#[thespian::actor]
impl MatchController {
    pub fn join(&mut self, controller: ClientControllerProxy, seat: Wind) -> Result<LocalState> {
        if self.clients.contains_key(&seat) {
            bail!("Seat is already occupied");
        }

        self.clients.insert(seat, ClientProxy::Client(controller));

        // If the expected number of players has joined the match, start the match once we
        // finish processing the current message.
        //
        // NOTE: We send a message to the actor to start the match rather than performing
        // the match start logic here because we want to make sure we send the initial match
        // state to the joining client before we broadcast any state updates.
        if self.clients.len() as u8 == self.num_players {
            self.remote
                .proxy()
                .start_match()
                .expect("Failed to send self a message");
        }

        Ok(self.state.local_state_for_player(seat))
    }

    /// Returns the updated match state if the requested discard is valid.
    #[tracing::instrument(skip(self))]
    pub fn discard_tile(&mut self, player: Wind, tile: TileId) -> Result<()> {
        trace!("Attempting to discard tile");

        // TODO: Provide more robust state transitions such that it's not possible to get
        // this far after the match has ended, e.g. a `MatchControllerState` enum that has
        // different states for whether the match is ongoing or completed.
        if self.state.wall.is_empty() {
            bail!("Match already finished");
        }

        // TODO: Verify that the client submitting the action is actually the one that
        // controls the player.

        self.state.discard_tile(player, tile)?;

        trace!("Successfully discarded tile");

        match &self.state.turn_state {
            // If any players can call the discarded tile, include the list of possible calls
            // when notifying them of the discard.
            TurnState::AwaitingCalls { waiting, .. } => {
                for seat in Wind::iter() {
                    let calls = waiting.get(&seat).cloned().unwrap_or_default();
                    self.clients
                        .get_mut(&seat)
                        .unwrap()
                        .send_event(MatchEvent::TileDiscarded {
                            seat: player,
                            tile,
                            calls,
                        })
                        .expect("Failed to send match update to client");
                }
            }

            &TurnState::AwaitingDraw(next_player) => {
                self.broadcast(MatchEvent::TileDiscarded {
                    seat: player,
                    tile,
                    calls: Default::default(),
                });

                let draw = self.state.draw_for_player(next_player)?;
                self.broadcast(MatchEvent::TileDrawn {
                    seat: next_player,
                    tile: draw,
                });
            }

            TurnState::MatchEnded { .. } => {
                self.broadcast(MatchEvent::TileDiscarded {
                    seat: player,
                    tile,
                    calls: Default::default(),
                });

                self.broadcast(MatchEvent::MatchEnded);
            }

            // If we're not waiting on any calls, broadcast the discard event to all players.
            _ => panic!("Unexpected turn state: {:?}", self.state.turn_state),
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn call_tile(&mut self, player: Wind, call: Option<Call>) -> Result<()> {
        todo!()
    }

    #[tracing::instrument(skip(self))]
    fn start_match(&mut self) {
        for seat in Wind::iter() {
            // NOTE: We need to manually split the borrows of `self.state` and `self.remote`
            // here because closures can't capture disjoint fields currently. This can be
            // cleaned up once rustc supports this.
            // See https://github.com/rust-lang/rust/issues/53488
            let state = &self.state;
            let remote = &self.remote;
            self.clients.entry(seat).or_insert_with(|| {
                let dummy = DummyClient {
                    seat,
                    state: state.clone(),
                    controller: remote.proxy(),
                }
                .spawn();

                ClientProxy::Dummy(dummy)
            });
        }

        // Draw the first tile and broadcast to the connected clients.
        let seat = match &self.state.turn_state {
            &TurnState::AwaitingDraw(seat) => seat,

            _ => panic!(
                "Unexpected turn state at start of match: {:?}",
                self.state.turn_state,
            ),
        };

        let tile = self
            .state
            .draw_for_player(seat)
            .expect("Failed to draw for first player");

        self.broadcast(MatchEvent::TileDrawn { seat, tile });
    }
}

/// Actor that controls players that aren't controlled by an active client.
#[derive(Debug, Actor)]
struct DummyClient {
    seat: Wind,
    state: MatchState,
    controller: MatchControllerProxy,
}

#[thespian::actor]
impl DummyClient {
    fn send_event(&mut self, event: MatchEvent) {
        match event {
            MatchEvent::TileDrawn { seat, tile } => {
                let draw = self.state.draw_for_player(seat).unwrap();
                assert_eq!(tile, draw);

                // If the player we control drew the tile, select a tile to discard.
                if seat == self.seat {
                    let _ = self
                        .controller
                        .discard_tile(self.seat, self.state.player(self.seat).tiles()[0].id)
                        .unwrap();
                }
            }

            MatchEvent::TileDiscarded { seat, tile, calls } => {
                self.state.discard_tile(seat, tile).unwrap();

                if !calls.is_empty() {
                    let _ = self
                        .controller
                        .call_tile(self.seat, Some(calls[0].clone()))
                        .unwrap();
                }
            }

            MatchEvent::Call {
                caller,
                winning_call,
                ..
            } => {
                self.state.call_tile(caller, Some(winning_call)).unwrap();
            }

            MatchEvent::MatchEnded => todo!("Figure out how to shut down actors?"),
        }
    }
}

/// Abstraction over either a concrete client actor or a dummy client actor.
// TODO: Remove this once thespian has support for actor traits.
// Tracking issue: https://github.com/randomPoison/thespian/issues/15
#[derive(Debug, Clone)]
enum ClientProxy {
    Client(ClientControllerProxy),
    Dummy(DummyClientProxy),
}

impl ClientProxy {
    pub fn send_event(&mut self, event: MatchEvent) -> Result<(), thespian::MessageError> {
        match self {
            ClientProxy::Client(proxy) => proxy.send_event(event),
            ClientProxy::Dummy(proxy) => proxy.send_event(event),
        }
    }
}
