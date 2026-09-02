use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Lit, Field, Fields, parse_macro_input};

fn get_doc_comment(field: &syn::Field) -> Option<String> {
    for attr in &field.attrs {
        if attr.path().is_ident("doc")
            && let Ok(meta) = attr.meta.require_name_value()
                && let Expr::Lit(expr_lit) = &meta.value
                    && let Lit::Str(lit) = &expr_lit.lit 
        {
            return Some(lit.value());
        }
    }
    None
}

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(Docs)]
pub fn docs_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

        let fields = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields_named) => {
                    fields_named.named.iter().collect::<Vec<&Field>>()
                }
                Fields::Unnamed(fields_unnamed) => {
                    fields_unnamed.unnamed.iter().collect::<Vec<&Field>>()
                }
                Fields::Unit => {
                    Vec::new()
                }
            }
        }
        _ => {
            panic!("Docs derive only works for structs");
        }
    };

    let mut lines = Vec::with_capacity(fields.len());
    for (i, field) in fields.iter().enumerate() {
        let field_name = field.ident.clone().unwrap().to_string();
        let idx = i + 1;
        let doc = get_doc_comment(field).unwrap_or_else(|| "Empty".to_owned());
        lines.push(format!("{field_name:>20} {:<4} : {doc}", format!("{{{idx}}}")));
    }
        
    let expanded = if lines.is_empty() {
        quote! {
            impl Docs for #struct_name {
                fn print_format() {
                    crate::println!("Module `{}` doesn't support output formatting", stringify!(#struct_name));
                }
            }
        }
    } else {
        quote! {
            impl Docs for #struct_name {
                fn print_format() {
                    crate::println!("The following variables are passed:");
                    #(
                        crate::println!("{}", #lines);
                    )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}