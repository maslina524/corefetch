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

fn field_to_config_name(field: &Field) -> String {
    field.ident.clone().unwrap().to_string().trim_start_matches("r#").replace('_', "-")
}

fn get_fields(data: &Data) -> Vec<&Field> {
    match data {
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
    }
}

// fn snake_to_camel_ascii(s: &str) -> String {
//     let mut ret = String::with_capacity(s.len());
//     let chars = s.chars();
//     let mut transition = false;

//     for ch in chars {
//         if ch == '_' || ch == '-' {
//             transition = true;
//             continue;
//         }
//         if transition {
//             if ch.is_ascii_lowercase() {
//                 let idx = ch as u32 - 32;
//                 ret.push(char::from_u32(idx).unwrap());
//             } else {
//                 ret.push(ch);
//             }

//             transition = false;
//             continue;
//         }
        
//         ret.push(ch);
//     }

//     ret
// }

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(Docs)]
pub fn docs_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let fields = get_fields(&input.data);

    // BUILD -h module-format
    let format = if fields.is_empty() {
        quote! {
            fn strings_format() -> Option<&'static [crate::DocString]> {
                None
            }
        }
    } else {
        let mut idx = 1;
        let strings = fields.iter().map(|field| {
            let name = field_to_config_name(field);
            let typ = format!("{{{idx}}}");
            let desc = match get_doc_comment(field) {
                Some(s) => quote! { Some(#s) },
                None => quote! { None }
            };
            idx += 1;

            quote! {
                crate::DocString { name: #name, second: #typ, desc: #desc }
            }
        });

        quote! {
            fn strings_format() -> Option<&'static [crate::DocString]> {
                Some(&[
                    #(#strings),*
                ])
            }
        }
    };

    // BUILD -h module-lua
    let lua = if fields.is_empty() {
        quote! {
            fn strings_lua() -> Option<&'static [crate::DocString]> {
                None
            }
        }
    } else {
        let strings = fields.iter().map(|field| {
            let name = field_to_config_name(field);
            let field_ty = &field.ty;
            let desc = match get_doc_comment(field) {
                Some(s) => quote! { Some(#s) },
                None => quote! { None }
            };

            quote! {
                crate::DocString { name: #name, second: <#field_ty as crate::lua::AsLua>::LUA_TYPE, desc: #desc }
            }
        });
        quote! {
            fn strings_lua() -> Option<&'static [crate::DocString]> {
                Some(&[
                    #(#strings),*
                ])
            }
        }
    };

    let combined = quote! {
        use crate::lua::AsLua;

        impl crate::Docs for #struct_name {
            #format
            #lua
        }
    };
    TokenStream::from(combined)
}