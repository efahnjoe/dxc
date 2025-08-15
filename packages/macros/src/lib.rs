use proc_macro::TokenStream;
use crate::classes::ClassListInput;

mod props;
mod classes;

#[proc_macro]
pub fn props(input: TokenStream) -> TokenStream {
    props::impl_props(input)
}

#[proc_macro]
pub fn classes(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ClassListInput);
    input.into_token_stream().into()
}