use proc_macro::{TokenStream, Span};
use syn::{parse_macro_input, Item, Error};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let _ = input;

    // eprintln!("args : {:#?}", args);
    // eprintln!("input: {:#?}", input);
    let item = parse_macro_input!(input as Item);
    eprintln!("item : {:#?}", Span::call_site());

    if let Item::Enum(_values) = item {

    } else {
        // compile error
        return Error::new(proc_macro2::Span::call_site(), "expected enum or match expression").into_compile_error().into();
    }
    TokenStream::new()
}
