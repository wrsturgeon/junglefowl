//! Macros to make interesting types to reason with.

#![deny(warnings)]
#![warn(
    missing_docs,
    rustdoc::all,
    clippy::missing_docs_in_private_items,
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::pub_use,
    clippy::question_mark_used,
    clippy::implicit_return,
    clippy::mod_module_files
)]

extern crate proc_macro;

use quote::ToTokens;

/// Create a unique type representing the given number.
#[proc_macro]
pub fn peano(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match perform(ts.into()) {
        Ok(done) => done,
        Err(e) => e.into_compile_error(),
    }
    .into()
}

/// Create a `syn`-compatible parentheses token.
macro_rules! paren {
    () => {
        syn::token::Paren {
            span: proc_macro2::Group::new(
                proc_macro2::Delimiter::Parenthesis,
                proc_macro2::TokenStream::new(),
            )
            .delim_span(),
        }
    };
}

/// Create a `syn`-compatible `()` token.
macro_rules! unit {
    () => {
        syn::Expr::Tuple(syn::ExprTuple {
            attrs: vec![],
            paren_token: paren!(),
            elems: syn::punctuated::Punctuated::new(),
        })
    };
}

/// Actually run the proc-macro
fn perform(ts: proc_macro2::TokenStream) -> Result<proc_macro2::TokenStream, syn::Error> {
    let arity_lit: syn::LitInt = syn::parse2(ts)?;
    let arity: isize = arity_lit.base10_parse()?;
    if arity < 0 {
        Err(syn::Error::new(
            arity_lit.span(),
            "Arity must be nonnegative",
        ))
    } else {
        let mut ast = unit!();
        #[allow(clippy::as_conversions, clippy::cast_sign_loss)]
        for _ in 0..(arity as usize) {
            ast = syn::Expr::Tuple(syn::ExprTuple {
                attrs: vec![],
                paren_token: paren!(),
                elems: {
                    let mut p = syn::punctuated::Punctuated::new();
                    p.push_value(unit!());
                    p.push_punct(syn::token::Comma {
                        spans: [proc_macro2::Span::call_site()],
                    });
                    p.push_value(ast);
                    p
                },
            });
        }
        Ok(ast.into_token_stream())
    }
}
