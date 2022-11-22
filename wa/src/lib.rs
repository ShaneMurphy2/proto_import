use proc_macro::TokenStream;
use watt::WasmMacro;

static MACRO: WasmMacro = WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("../../impl/target/wasm32-unknown-unknown/release/proto_import.wasm");

/// Import `prost` generated definitions.
///
/// # Examples
///
/// Given a .proto file with a package `foo.bar`, you would write
/// ```
/// # use proto_import::import;
///
/// import!(foo::bar);
/// ```
/// which will expand to:
/// ```
/// pub mod foo {
///     pub mod bar {
///         include!(concat!(env!("OUT_DIR"), "/foo.bar.rs"));
///     }
/// }
/// ```
#[proc_macro]
pub fn import(input: TokenStream) -> TokenStream {
    MACRO.proc_macro("import", input)
}

