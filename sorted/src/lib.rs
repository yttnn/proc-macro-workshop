use proc_macro::TokenStream;
use syn::{parse_macro_input, Item, Error};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let _ = input;

    // eprintln!("args : {:#?}", args);
    // eprintln!("input: {:#?}", input);
    let item = parse_macro_input!(input as Item);
    // eprintln!("item : {:#?}", Span::call_site());

    let item_enum = if let Item::Enum(item_enum) = item {
        item_enum
    } else {
        // compile error
        return Error::new(proc_macro2::Span::call_site(), "expected enum or match expression").into_compile_error().into();
    };

    let variant_idents: Vec<proc_macro2::Ident> = item_enum.variants.iter().map(|v| v.ident.clone()).collect();

    // dbg!(variant_idents);
    let mut variant_idents_sorted = variant_idents.clone();
    variant_idents_sorted.sort();

    for (original, sorted) in variant_idents.iter()
                                        .zip(variant_idents_sorted.iter())
    {
        if original != sorted {
            return Error::new(sorted.span(), format!("{} should sort before {}", sorted, original))
            .into_compile_error().into();
        }
    }

    TokenStream::new()
}
