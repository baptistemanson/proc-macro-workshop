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
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
