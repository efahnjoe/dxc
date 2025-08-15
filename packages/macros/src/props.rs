use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Ident, Token, Type};

pub struct PropsMacroInput {
    pub name: Ident,
    pub fields: Vec<(Ident, Type)>,
    pub extra_derives: Vec<Ident>,
}

impl Parse for PropsMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let content;
        syn::braced!(content in input);

        let fields = content.parse_terminated(|input| {
            let field = input.parse::<Ident>()?;
            input.parse::<Token![:]>()?;
            let ty = input.parse::<Type>()?;
            Ok((field, ty))
        }, Token![,])?;

        let mut extra_derives = Vec::new();

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.peek(Ident) {
                let ident: Ident = input.parse()?;
                if ident.to_string() == "derive" {
                    let derive_content;
                    syn::bracketed!(derive_content in input);
                    let derives_punctuated = derive_content.parse_terminated(Ident::parse, Token![,])?;
                    extra_derives = derives_punctuated.into_iter().collect();
                } else {
                    return Err(syn::Error::new(ident.span(), "expected `derive`"));
                }
            }
        }

        Ok(PropsMacroInput {
            name,
            fields: fields.into_iter().collect(),
            extra_derives,
        })
    }
}

pub fn impl_props(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PropsMacroInput);

    let struct_name = input.name;
    let fields = input.fields;
    let extra_derives = input.extra_derives;

    // 基础 derive
    let mut all_derives = vec![
        quote!(Clone),
        quote!(Props),
        quote!(PartialEq),
        quote!(Debug),
    ];
    all_derives.extend(extra_derives.iter().map(|d| quote!(#d)));

    let field_definitions = fields.iter().map(|(ident, ty)| {
        quote! {
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