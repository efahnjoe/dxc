use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Error, Field, Ident, Result, Token,Path
};

pub struct PropsMacroInput {
    pub name: Ident,
    pub fields: Vec<Field>,
    pub extra_derives: Vec<syn::Path>,
}

impl Parse for PropsMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        let content;
        braced!(content in input);

        let fields_punctuated: Punctuated<Field, Comma> =
            content.parse_terminated(Field::parse_named, Token![,])?;

        let mut extra_derives = Vec::new();

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.peek(Ident) {
                let ident: Ident = input.parse()?;
                if ident.to_string() == "derive" {
                    let derive_content;
                    parenthesized!(derive_content in input);

                    // Usage of Path::parse_mod_style
                    // like Serialize or serde::Serialize
                    let derives_punctuated =
                        derive_content.parse_terminated(Path::parse_mod_style, Token![,])?;
                    extra_derives = derives_punctuated.into_iter().collect();
                } else {
                    return Err(Error::new(ident.span(), "expected `derive`"));
                }
            }
        }

        let fields = fields_punctuated.into_iter().collect();

        Ok(PropsMacroInput {
            name,
            fields,
            extra_derives,
        })
    }
}

pub fn impl_props(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PropsMacroInput);

    let struct_name = input.name;
    let fields = input.fields;
    let extra_derives = input.extra_derives;

    let mut all_derives = vec![
        quote!(Clone),
        quote!(Props),
        quote!(Default),
        quote!(PartialEq),
        quote!(Debug),
    ];
    all_derives.extend(extra_derives.iter().map(|d| quote!(#d)));

    let field_definitions = fields.iter().map(|field| {
        let attrs = &field.attrs;
        let ident = &field.ident.as_ref().unwrap();
        let ty = &field.ty;

        quote! {
            #(#attrs)*
            #[props(default)]
            pub #ident: Option<#ty>
        }
    });

    let expanded = quote! {
        #[derive(#(#all_derives),*)]
        pub struct #struct_name {
            #(#field_definitions,)*
        }
    };

    TokenStream::from(expanded)
}
