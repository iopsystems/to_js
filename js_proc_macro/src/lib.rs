extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn js(_: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree of a function item
    let mut item_fn: ItemFn = parse_macro_input!(input);

    // The to_js! macro requires functions to have an explicit return type,
    // so add it if one doesn't already exist.
    // Check if the function has an explicit return type
    if let ReturnType::Default = item_fn.sig.output {
        // If not, set the return type to an empty tuple `()`
        item_fn.sig.output = ReturnType::Type(
            syn::token::RArrow::default(),
            Box::new(syn::parse_quote!(())),
        );
    }

    // Apply the macro_rules! macro to the function item's tokens
    let expanded = quote! {
        to_js::to_js! {
            #item_fn
        }
    };

    // Parse the expanded tokens back into a TokenStream
    expanded.into()
}
