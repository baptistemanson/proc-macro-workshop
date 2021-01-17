use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

fn is_option(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(p) => p.path.segments[0].ident == "Option",
        _ => false,
    }
}

fn get_option_inner_type(ty: &syn::Type) -> &syn::Type {
    match ty {
        syn::Type::Path(p) => match &p.path.segments[0].arguments {
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => {
                if let syn::GenericArgument::Type(optional_type) = &args[0] {
                    optional_type
                } else {
                    panic!("problem finding the type inside the option")
                }
            }
            _ => panic!("only works on option type"),
        },
        _ => panic!("only works on option type"),
    }
}

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let original_id = input.ident;
    let s = match input.data {
        Data::Struct(s) => s,
        _ => panic!("Only accept struct"),
    };

    let builder_id = format_ident!("{}Builder", original_id);
    let mut field_name: Vec<TokenStream> = vec![];
    let mut field_type: Vec<TokenStream> = vec![];
    let mut field_setter_type: Vec<TokenStream> = vec![];
    let mut field_setter_expression: Vec<TokenStream> = vec![];
    let mut field_builder_expression: Vec<TokenStream> = vec![];

    s.fields.iter().for_each(|s| {
        let name = &s.ident;
        let ty = &s.ty;
        let is_field_option = is_option(&s.ty);

        // struct
        field_name.push(quote! {#name});
        field_type.push(quote! {#ty});

        // set
        let setter_type = if is_field_option {
            &get_option_inner_type(&s.ty)
        } else {
            &s.ty
        };
        field_setter_type.push(quote! { #setter_type});
        // @todo add vector stuff here
        field_setter_expression.push(quote! { self.#name = ::std::option::Option::Some(#name)});

        // build
        let builder_expression = if is_field_option {
            quote! { let #name = self.#name.clone()}
        } else {
            quote! { let #name = self.#name.clone().ok_or("error")?}
        };
        field_builder_expression.push(builder_expression);
    });

    let expanded = quote! {
    #[derive(Debug)]
            pub struct #builder_id {
                #(#field_name: ::std::option::Option<#field_setter_type>,)*
            }

            impl #original_id {
                 fn builder() -> #builder_id {
                    #builder_id {
                        #(#field_name : None,)*
                    }
                }
            }

            impl #builder_id {
                #(pub fn #field_name(&mut self, #field_name: #field_setter_type) -> &mut Self {
                    #field_setter_expression;
                    self
                })*

                pub fn build(&mut self) -> ::std::result::Result<#original_id, ::std::boxed::Box<dyn ::std::error::Error>> {
                    #(#field_builder_expression;)*
                    ::std::result::Result::Ok(#original_id {
                        #(#field_name,)*
                    })
                }
            }
        };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}
