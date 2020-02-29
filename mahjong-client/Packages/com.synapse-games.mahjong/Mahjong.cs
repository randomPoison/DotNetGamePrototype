using System ; using System . Runtime . InteropServices ; using System . Text ; internal unsafe static class __bindings { [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_drop_string" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern void __cs_bindgen_drop_string ( RustOwnedString raw ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_drop__ClientState" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern void __cs_bindgen_drop__ClientState ( void * self ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_generated__new" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern void * __cs_bindgen_generated__new ( ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_generated__set_credentials" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern void __cs_bindgen_generated__set_credentials ( void * self , ulong id , RawCsString token ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_generated__create_handshake_request" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern RustOwnedString __cs_bindgen_generated__create_handshake_request ( void * self ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_generated__handle_handshake_response" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern byte __cs_bindgen_generated__handle_handshake_response ( void * self , RawCsString json ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_generated__create_start_match_request" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern RustOwnedString __cs_bindgen_generated__create_start_match_request ( void * self ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_generated__handle_start_match_response" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern void * __cs_bindgen_generated__handle_start_match_response ( void * self , RawCsString response ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_generated__account_id" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern ulong __cs_bindgen_generated__account_id ( void * self ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_generated__points" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern ulong __cs_bindgen_generated__points ( void * self ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_drop__SimpleTile" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern void __cs_bindgen_drop__SimpleTile ( void * self ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_drop__Match" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern void __cs_bindgen_drop__Match ( void * self ) ; [ DllImport ( "mahjong" , EntryPoint = "__cs_bindgen_generated__id" , CallingConvention = CallingConvention . Cdecl ) ] internal static extern uint __cs_bindgen_generated__id ( void * self ) ; } public class Mahjong { } public unsafe partial class ClientState : IDisposable { private void * _handle ; internal ClientState ( void * handle ) { _handle = handle ; } public void Dispose ( ) { if ( _handle != null ) { __bindings . __cs_bindgen_drop__ClientState ( _handle ) ; _handle = null ; } } } partial class ClientState { public ClientState ( ) { unsafe { _handle = __bindings . __cs_bindgen_generated__new ( ) ; } } } partial class ClientState { public void SetCredentials ( ulong id , string token ) { unsafe { fixed ( char * __fixed_token = token ) { __bindings . __cs_bindgen_generated__set_credentials ( this . _handle , id , new RawCsString ( __fixed_token , token . Length ) ) ; } } } } partial class ClientState { public string CreateHandshakeRequest ( ) { string __ret ; unsafe { var __raw_result = __bindings . __cs_bindgen_generated__create_handshake_request ( this . _handle ) ; __ret = Encoding . UTF8 . GetString ( __raw_result . Ptr , ( int ) __raw_result . Length ) ; __bindings . __cs_bindgen_drop_string ( __raw_result ) ; } return __ret ; } } partial class ClientState { public bool HandleHandshakeResponse ( string json ) { bool __ret ; unsafe { fixed ( char * __fixed_json = json ) { __ret = __bindings . __cs_bindgen_generated__handle_handshake_response ( this . _handle , new RawCsString ( __fixed_json , json . Length ) ) != 0 ; } } return __ret ; } } partial class ClientState { public string CreateStartMatchRequest ( ) { string __ret ; unsafe { var __raw_result = __bindings . __cs_bindgen_generated__create_start_match_request ( this . _handle ) ; __ret = Encoding . UTF8 . GetString ( __raw_result . Ptr , ( int ) __raw_result . Length ) ; __bindings . __cs_bindgen_drop_string ( __raw_result ) ; } return __ret ; } } partial class ClientState { public Match HandleStartMatchResponse ( string response ) { Match __ret ; unsafe { fixed ( char * __fixed_response = response ) { __ret = new Match ( __bindings . __cs_bindgen_generated__handle_start_match_response ( this . _handle , new RawCsString ( __fixed_response , response . Length ) ) ) ; } } return __ret ; } } partial class ClientState { public ulong AccountId ( ) { ulong __ret ; unsafe { __ret = __bindings . __cs_bindgen_generated__account_id ( this . _handle ) ; } return __ret ; } } partial class ClientState { public ulong Points ( ) { ulong __ret ; unsafe { __ret = __bindings . __cs_bindgen_generated__points ( this . _handle ) ; } return __ret ; } } public enum Suit { Coins , Bamboo , Characters } public unsafe partial class SimpleTile : IDisposable { private void * _handle ; internal SimpleTile ( void * handle ) { _handle = handle ; } public void Dispose ( ) { if ( _handle != null ) { __bindings . __cs_bindgen_drop__SimpleTile ( _handle ) ; _handle = null ; } } } public enum Flower { PlumBlossom , Orchid , Chrysanthemum , Bamboo } public enum Season { Spring , Summer , Autumn , Winter } public enum Wind { East , South , West , North } public enum Dragon { Red , Green , White } public unsafe partial class Match : IDisposable { private void * _handle ; internal Match ( void * handle ) { _handle = handle ; } public void Dispose ( ) { if ( _handle != null ) { __bindings . __cs_bindgen_drop__Match ( _handle ) ; _handle = null ; } } } partial class Match { public uint Id ( ) { uint __ret ; unsafe { __ret = __bindings . __cs_bindgen_generated__id ( this . _handle ) ; } return __ret ; } } [ StructLayout ( LayoutKind . Sequential ) ] internal unsafe struct RustOwnedString { public byte * Ptr ; public UIntPtr Length ; public UIntPtr Capacity ; } [ StructLayout ( LayoutKind . Sequential ) ] internal unsafe struct RawCsString { public char * Ptr ; public UIntPtr Length ; public RawCsString ( char * ptr , UIntPtr len ) { Ptr = ptr ; Length = len ; } public RawCsString ( char * ptr , int len ) { Ptr = ptr ; Length = ( UIntPtr ) len ; } }