#![feature(proc_macro_diagnostic)]

use proc_macro::{Diagnostic, Level, Span, TokenStream};
use syn::LitStr;

#[proc_macro_attribute]
#[allow(clippy::missing_panics_doc)]
pub fn todo(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message = syn::parse::<LitStr>(attr).map_or_else(
        |_| "TODO: Not implemented".to_owned(),
        |l| format!("TODO: {}", l.value()),
    );

    let span = Span::mixed_site();
    Diagnostic::spanned(span, Level::Warning, &message).emit();

    item
}