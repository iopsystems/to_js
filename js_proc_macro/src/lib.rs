use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, ItemFn, LitStr, Result, ReturnType,
};
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

struct JsArgs {
    prefix: Option<String>,
}

impl Parse for JsArgs {
    /// Parses the args in #[js(prefix = "foo_")] to allow prefixing a function name.
    /// This is useful if you want to define a macro to define the same set of exports
    /// for multiple structs.
    /// Note that this will prefix not only the export but also the name of the function
    /// on the Rust side, avoiding name collisions if a function with the same name is
    /// exported multiple times, as it might be during a macro-driven generation process.
    fn parse(input: ParseStream) -> Result<Self> {
        let mut name_prefix = None;

        // Parse the input stream to extract prefix argument if provided
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let ident: Ident = input.parse()?;
                let _eq_token: syn::Token![=] = input.parse()?;
                let value: LitStr = input.parse()?;
                if ident == "name_prefix" {
                    let mut s = value.value();
                    if s.ends_with('_') {
                        return Err(syn::Error::new(
                            ident.span(),
                            "name prefix should not include a trailing underscore, as one will be added automatically",
                        ));
                    }
                    s.push('_');
                    name_prefix = Some(s);
                }
            }
            if input.peek(syn::Token![,]) {
                let _comma: syn::Token![,] = input.parse()?;
            }
        }

        Ok(JsArgs {
            prefix: name_prefix,
        })
    }
}

#[proc_macro_attribute]
pub fn js(attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree of a function item
    let mut item_fn: ItemFn = parse_macro_input!(input);

    // Parse the attribute arguments
    let args = parse_macro_input!(attr as JsArgs);

    // Modify the function name by adding the prefix if provided
    if let Some(prefix) = args.prefix {
        let original_name = item_fn.sig.ident.to_string();
        let new_name = format!("{}{}", prefix, original_name);
        item_fn.sig.ident = Ident::new(&new_name, item_fn.sig.ident.span());
    }

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
