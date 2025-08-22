use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, token::Paren, Attribute, Data,
    DeriveInput, Error, Expr, ExprCall, ExprLit, ExprPath, Fields, GenericArgument, Ident, Lit,
    LitStr, PathArguments, Result as SynResult, Type,
};

pub struct PropsDefaultInput {
    struct_name: Ident,
    fields: Vec<FieldInfo>,
}

struct FieldInfo {
    name: Ident,
    inner_type: Type,
    skip: bool,
    custom_default: Option<Expr>,
}

impl PropsDefaultInput {
    pub fn from_derive_input(input: DeriveInput) -> SynResult<Self> {
        let struct_name = input.ident;

        let fields = match &input.data {
            Data::Struct(s) => match &s.fields {
                Fields::Named(named) => &named.named,
                _ => {
                    return Err(Error::new(
                        struct_name.span(),
                        "Only named fields are supported",
                    ))
                }
            },
            _ => return Err(Error::new(struct_name.span(), "Only structs are supported")),
        };

        fields
            .iter()
            .filter_map(|field| {
                let name = field.ident.clone()?;
                // Get inner type without cloning original type
                let inner_type = get_inner_type(&field.ty, "Option")?;
                Some((name, field, inner_type))
            })
            .map(|(name, field, inner_type)| {
                let (skip, custom_default) = parse_field_attrs(&field.attrs)?;
                Ok(FieldInfo {
                    name,
                    inner_type,
                    skip,
                    custom_default,
                })
            })
            .collect::<SynResult<Vec<FieldInfo>>>()
            .map(|field_infos| Self {
                struct_name,
                fields: field_infos,
            })
    }
}

fn get_inner_type(ty: &Type, wrapper: &str) -> Option<Type> {
    if let Type::Path(p) = ty {
        if p.path.segments.len() == 1 && p.path.segments[0].ident == wrapper {
            if let PathArguments::AngleBracketed(args) = &p.path.segments[0].arguments {
                if let Some(GenericArgument::Type(ty)) = args.args.first() {
                    return Some(ty.clone());
                }
            }
        }
    }
    None
}

fn parse_field_attrs(attrs: &[Attribute]) -> SynResult<(bool, Option<Expr>)> {
    let mut skip = false;
    let mut custom_default = None;
    let mut props_default_span = None;

    for attr in attrs {
        if !attr.path().is_ident("props_default") {
            continue;
        }

        let span = attr.span().into();
        if props_default_span.replace(span).is_some() {
            return Err(Error::new(span, "Duplicate `props_default` attribute"));
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                if skip {
                    return Err(meta.error("Duplicate `skip`"));
                }
                skip = true;
            } else if meta.path.is_ident("value") {
                if custom_default.is_some() {
                    return Err(meta.error("Duplicate `value`"));
                }

                let value_expr = meta.value()?.parse::<Expr>()?;

                let expr_converted = match &value_expr {
                    Expr::Lit(lit) => {
                        if let Lit::Str(str_lit) = &lit.lit {
                            // Construct the Expr AST of String::from("button")
                            Expr::Call(ExprCall {
                                attrs: Vec::new(),
                                func: Box::new(Expr::Path(ExprPath {
                                    attrs: Vec::new(),
                                    qself: None,
                                    path: parse_quote!(::std::string::String::from),
                                })),
                                paren_token: Paren::default(),
                                args: {
                                    let mut args = Punctuated::new();
                                    args.push(Expr::Lit(ExprLit {
                                        attrs: Vec::new(),
                                        lit: Lit::Str(LitStr::new(
                                            &str_lit.value(),
                                            str_lit.span(),
                                        )),
                                    }));
                                    args
                                },
                            })
                        } else {
                            value_expr.clone()
                        }
                    }
                    _ => value_expr.clone(),
                };

                custom_default = Some(expr_converted);
            } else {
                return Err(meta.error("Unsupported attribute"));
            }
            Ok(())
        })?;
    }

    if skip && custom_default.is_some() {
        return Err(Error::new(
            props_default_span.unwrap_or_else(Span::call_site),
            "Cannot specify both `skip` and `value`",
        ));
    }

    Ok((skip, custom_default))
}

impl From<PropsDefaultInput> for TokenStream {
    fn from(input: PropsDefaultInput) -> Self {
        let struct_name = input.struct_name;
        let methods = input.fields.iter().filter(|f| !f.skip).map(|field| {
            let field_name = &field.name;
            let method_name = Ident::new(&field_name.to_string(), field_name.span());
            let inner_type = &field.inner_type;

            match &field.custom_default {
                Some(expr) => quote! {
                    pub fn #method_name(&self) -> #inner_type {
                        self.#field_name.clone().unwrap_or_else(|| #expr)
                    }
                },
                None => quote! {
                    pub fn #method_name(&self) -> #inner_type {
                        self.#field_name.clone().unwrap_or_default()
                    }
                },
            }
        });

        quote! { impl #struct_name { #(#methods)* } }
    }
}
