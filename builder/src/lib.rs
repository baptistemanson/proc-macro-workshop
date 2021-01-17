use core::panic;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let original_id = input.ident;
    let s = match input.data {
        Data::Struct(s) => s,
        _ => panic!("Only accept struct"),
    };

    let builder_id = format_ident!("{}Builder", original_id);

    let field_names: Vec<proc_macro2::TokenStream> = s
        .fields
        .iter()
        .map(|s| {
            let name = &s.ident;
            quote! { #name }
        })
        .collect();

    let field_types: Vec<proc_macro2::TokenStream> = s
        .fields
        .iter()
        .map(|s| {
            let ty = &s.ty;
            quote! { #ty }
        })
        .collect();

    let expanded = quote! {
    #[derive(Debug)]
    struct BuilderError {}

    impl std::fmt::Display for BuilderError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BuilderError is here!")
        }
    }
    impl std::error::Error for BuilderError {
        fn description(&self) -> &str {
            "invalid utf-16"
        }
    }

            pub struct #builder_id {
                #(#field_names: Option<#field_types>,)*
            }

            impl #original_id {
                 fn builder() -> #builder_id {
                    #builder_id {
                        #(#field_names : None,)*
                    }
                }
            }


            impl #builder_id {
                #(pub fn #field_names(&mut self, #field_names: #field_types) -> &mut Self {
                    self.#field_names = Some(#field_names);
                    self
                })*

                pub fn build(&mut self) -> Result<#original_id, Box<dyn std::error::Error>> {
                    #(let #field_names = self.#field_names.clone().ok_or(BuilderError {})?;)*
                    Ok(#original_id {
                        #(#field_names: #field_names.clone(),)*
                    })
                }
            }
        };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
