use core::panic;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let original_id = input.ident;
    let s = match input.data {
        Data::Struct(s) => s,
        _ => panic!("Only accept struct"),
    };

    let builder_id = Ident::new(&format!("{}Builder", original_id), Span::call_site());
    let builder_fields: Vec<proc_macro2::TokenStream> = s
        .fields
        .iter()
        .map(|s| {
            let name = &s.ident;
            let ty = &s.ty;
            quote! { #name: Option<#ty> }
        })
        .collect();

    let default_builder_fields_values: Vec<proc_macro2::TokenStream> = s
        .fields
        .iter()
        .map(|s| {
            let name = &s.ident;
            quote! { #name: None }
        })
        .collect();

    let expanded = quote! {
       impl #original_id {
         fn builder() -> #builder_id {
            #builder_id {
                #(#default_builder_fields_values,)*
            }
         }
       }
        pub struct #builder_id {
            #(#builder_fields,)*
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
