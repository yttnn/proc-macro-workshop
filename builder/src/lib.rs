use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
// use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    dbg!(ast.data);
    TokenStream::new()
}
