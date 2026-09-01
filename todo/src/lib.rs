#![cfg_attr(unstable, feature(proc_macro_diagnostic))]
#![cfg_attr(unstable, feature(proc_macro_value))]

#[cfg(unstable)]
use proc_macro::{Diagnostic, Level, Span, TokenStream, TokenTree};

#[cfg(not(unstable))]
use proc_macro::TokenStream;

fn get_message(attr: TokenStream) -> Option<String> {
    attr.into_iter().next().and_then(|t| match t {
        TokenTree::Literal(s) => {
            let string = s.str_value().ok()?;
            Some(format!("TODO: {string}"))
        },
        _ => None
    })
}

#[allow(clippy::missing_panics_doc)]
#[cfg(unstable)]
#[proc_macro_attribute]
pub fn todo(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message = get_message(attr)
        .unwrap_or_else(|| "TODO: Not implemented".to_owned());

    let span = Span::mixed_site();
    Diagnostic::spanned(span, Level::Warning, &message).emit();

    item
}

#[cfg(not(unstable))]
#[proc_macro_attribute]
pub fn todo(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}