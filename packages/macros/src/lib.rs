use crate::classes::ClassListInput;
use crate::props_default::PropsDefaultInput;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput};
mod classes;
mod props;
mod props_default;

#[proc_macro]
pub fn props(input: TokenStream) -> TokenStream {
    props::impl_props(input.into()).into()
}

#[proc_macro]
pub fn classes(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ClassListInput);
    input.into_token_stream().into()
}

#[proc_macro_derive(PropsDefault, attributes(props_default))]
pub fn props_default_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let props_input = PropsDefaultInput::from_derive_input(ast);

    match props_input {
        Ok(input) => {
            let tokens: TokenStream2 = input.into();
            tokens.into()
        }
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
