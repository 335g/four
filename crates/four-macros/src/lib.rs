use four_core::Resource;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput, Item, ItemFn};

#[proc_macro_attribute]
pub fn stack(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Item);

    if let Item::Fn(item_fn) = input {
        let ItemFn {
            attrs,
            vis,
            sig,
            block,
        } = item_fn;

        todo!()
    } else {
        syn::Error::new(Span::call_site(), "expected function")
            .to_compile_error()
            .into()
    }
}
