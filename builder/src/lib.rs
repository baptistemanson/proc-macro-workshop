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

    let param_names: Vec<proc_macro2::TokenStream> = s
        .fields
        .iter()
        .map(|s| {
            let name = &s.ident;
            quote! { #name }
        })
        .collect();

    let types: Vec<proc_macro2::TokenStream> = s
        .fields
        .iter()
        .map(|s| {
            let ty = &s.ty;
            quote! { #ty }
        })
        .collect();

    let expanded = quote! {
        pub struct #builder_id {
            #(#param_names: Option<#types>,)*
        }

        impl #original_id {
             fn builder() -> #builder_id {
                #builder_id {
                    #(#param_names : None,)*
                }
            }
        }


        impl #builder_id {
            #(pub fn #param_names(&mut self, #param_names: #types) -> &mut Self {
                self.#param_names = Some(#param_names);
                self
            })*
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
