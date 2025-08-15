use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, ExprIf, ExprLit, Lit, Token,
};

pub struct ClassListInput(Vec<ClassPart>);

impl Parse for ClassListInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let exprs = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        let parts = exprs.into_iter().map(ClassPart::from_expr).collect();
        Ok(ClassListInput(parts))
    }
}

impl ClassListInput {
    pub fn into_token_stream(self) -> proc_macro2::TokenStream {
        let stmts = self.0.into_iter().map(|part| part.into_stmt());
        quote! {{
            let mut classes = String::new();
            #(#stmts)*
            classes.trim_end().to_string()
        }}
    }
}

enum ClassPart {
    Literal(String),
    Conditional(ExprIf),
    Expression(Expr),
}

impl ClassPart {
    fn from_expr(expr: Expr) -> Self {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) => Self::Literal(s.value()),
            Expr::Lit(ExprLit {
                lit: Lit::Bool(b), ..
            }) => Self::Literal(b.value.to_string()),
            Expr::Lit(ExprLit {
                lit: Lit::Int(i), ..
            }) => Self::Literal(i.base10_parse::<u64>().unwrap().to_string()),
            Expr::If(expr_if) => Self::Conditional(expr_if),
            other => Self::Expression(other),
        }
    }

    fn into_stmt(self) -> proc_macro2::TokenStream {
        match self {
            Self::Literal(s) => {
                if s.is_empty() {
                    quote! {}
                } else {
                    quote! {
                        classes.push_str(#s);
                        classes.push(' ');
                    }
                }
            }
            Self::Conditional(expr_if) => {
                let cond = &expr_if.cond;
                let then_branch = &expr_if.then_branch;
                let else_branch = &expr_if.else_branch;

                let then_exprs = then_branch.stmts.iter().filter_map(|stmt| {
                    if let syn::Stmt::Expr(expr, _) = stmt {
                        Some(quote! {
                            classes.push_str(#expr);
                            classes.push(' ');
                        })
                    } else {
                        None
                    }
                });

                let else_clause = if let Some((_, else_expr)) = else_branch {
                    match &**else_expr {
                        Expr::Block(block) => {
                            let else_stmts = &block.block.stmts;
                            let else_exprs = else_stmts.iter().filter_map(|stmt| {
                                if let syn::Stmt::Expr(expr, _) = stmt {
                                    Some(quote! {
                                        classes.push_str(#expr);
                                        classes.push(' ');
                                    })
                                } else {
                                    None
                                }
                            });
                            quote! { else { #(#else_exprs)* } }
                        }
                        Expr::Lit(lit) => {
                            quote! {
                                else {
                                    classes.push_str(#lit);
                                    classes.push(' ');
                                }
                            }
                        }
                        _ => quote! {},
                    }
                } else {
                    quote! {}
                };

                quote! {
                    if #cond {
                        #(#then_exprs)*
                    } #else_clause
                }
            }
            Self::Expression(expr) => {
                quote! {
                    {
                        let s = #expr; // 先求值
                        let s_str: &str = s.as_ref();
                        if !s_str.is_empty() {
                            classes.push_str(s_str);
                            classes.push(' ');
                        }
                    }
                }
            }
        }
    }
}