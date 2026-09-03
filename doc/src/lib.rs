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

fn snake_to_camel_ascii(s: &str) -> String {
    let mut ret = String::with_capacity(s.len());
    let chars = s.chars();
    let mut transition = false;

    for ch in chars {
        if ch == '_' || ch == '-' {
            transition = true;
            continue;
        }
        if transition {
            if ch.is_ascii_lowercase() {
                let idx = ch as u32 - 32;
                ret.push(char::from_u32(idx).unwrap());
            } else {
                ret.push(ch);
            }

            transition = false;
            continue;
        }
        
        ret.push(ch);
    }

    ret
}

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(Docs)]
pub fn docs_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let fields = get_fields(&input.data);

    let mut lines = Vec::with_capacity(fields.len());
    for (i, field) in fields.iter().enumerate() {
        let field_name = field_to_config_name(field);
        let idx = i + 1;
        let doc = get_doc_comment(field).unwrap_or_else(|| "Empty".to_owned());
        lines.push(format!("{field_name:>20} : {:<4} : {doc}", format!("{{{idx}}}")));
    }

    // BUILD -h module-format  
    let format = if lines.is_empty() {
        quote! {
            fn print_format() {
                crate::println!("Module `{}` doesn't support output formatting", stringify!(#struct_name));
            }
        }
    } else {
        let first_field = field_to_config_name(fields.first().unwrap());
        quote! {
            fn print_format() {
                crate::println!(
                    "# In config file: {{ \"type\": \"{}\", \"format\": \"{{{}}} or {{1}}\" }}",
                    stringify!(#struct_name).to_lowercase(), #first_field
                );
                crate::println!("The following variables are passed:");
                #(
                    crate::println!("{}", #lines);
                )*
            }
        }
    };

    // BUILD -h module-lua
    let lua = if fields.is_empty() {
        quote! {
            fn print_lua() {
                crate::println!("Module `{}` doesn't support lua", stringify!(#struct_name));
            }
        }
    } else {
        let first_field = field_to_config_name(fields.first().unwrap());
        let first_field_camel = snake_to_camel_ascii(&first_field);
        let lua_stmts = fields.iter().map(|field| {
            let field_name = field_to_config_name(field);
            let field_ty = &field.ty;
            let doc = get_doc_comment(field).unwrap_or_else(|| "Empty".to_owned());
            quote! {
                crate::println!("{:>20} : {:<6} : {}", #field_name, #field_ty::lua_type(), #doc);
            }
        });
        
        quote! {
            fn print_lua() {
                crate::println!(
                    "# In config file: {{ \"type\": \"{}\", \"format\": \"lua: return (...).{}\" }}",
                    stringify!(#struct_name).to_lowercase(), #first_field_camel
                );
                crate::println!("The following variables are passed:");
                #(#lua_stmts)*
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