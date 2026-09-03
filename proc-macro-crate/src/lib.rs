use proc_macro::TokenStream;

// `native-dep-crate` is never actually called from here - it only needs to
// be in the dependency graph for cargo to compile it for the BUILD platform.
#[proc_macro]
pub fn answer(_input: TokenStream) -> TokenStream {
    "42".parse().unwrap()
}
