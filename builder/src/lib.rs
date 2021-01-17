use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

fn is_type_option(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(p) => p.path.segments[0].ident == "Option",
        _ => false,
    }
}

fn get_option_inner_type(ty: &syn::Type) -> syn::Type {
    match ty {
        syn::Type::Path(p) => match &p.path.segments[0].arguments {
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => {
                if let syn::GenericArgument::Type(optional_type) = &args[0] {
                    optional_type.clone()
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
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let original_id = input.ident;
    let s = match input.data {
        Data::Struct(s) => s,
        _ => panic!("Only accept struct"),
    };

    let builder_id = format_ident!("{}Builder", original_id);

    let all_fields: Vec<(bool, proc_macro2::TokenStream, &syn::Type)> = s
        .fields
        .iter()
        .map(|s| {
            let name = &s.ident;
            let ty = &s.ty;
            (is_type_option(&s.ty), quote! { #name }, ty)
        })
        .collect();
    let (all_names, _all_types): (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) =
        all_fields
            .iter()
            .map(|f| {
                let ty = f.2;
                (f.1.clone(), quote! { #ty })
            })
            .unzip();

    let (mandatory_names, mandatory_types): (
        Vec<proc_macro2::TokenStream>,
        Vec<proc_macro2::TokenStream>,
    ) = all_fields
        .iter()
        .filter_map(|f| {
            if f.0 {
                None
            } else {
                let ty = f.2;
                Some((f.1.clone(), quote! { #ty }))
            }
        })
        .unzip();

    let (optional_names, optional_inner_types): (
        Vec<proc_macro2::TokenStream>,
        Vec<proc_macro2::TokenStream>,
    ) = all_fields
        .iter()
        .filter_map(|f| {
            if !f.0 {
                None
            } else {
                let ty = get_option_inner_type(&f.2);
                Some((f.1.clone(), quote! { #ty}))
            }
        })
        .unzip();
    // identify all optional types and all non optional types
    // will be useful for the build function

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
                #(#mandatory_names: Option<#mandatory_types>,)*
                #(#optional_names: Option<#optional_inner_types>,)*
            }

            impl #original_id {
                 fn builder() -> #builder_id {
                    #builder_id {
                        #(#all_names : None,)*
                    }
                }
            }


            impl #builder_id {
                #(pub fn #mandatory_names(&mut self, #mandatory_names: #mandatory_types) -> &mut Self {
                    self.#mandatory_names = Some(#mandatory_names);
                    self
                })*

                #(pub fn #optional_names(&mut self, #optional_names: #optional_inner_types) -> &mut Self {
                    self.#optional_names = Some(#optional_names);
                    self
                })*

                pub fn build(&mut self) -> Result<#original_id, Box<dyn std::error::Error>> {
                    #(let #mandatory_names = self.#mandatory_names.clone().ok_or(BuilderError {})?;)*
                    #(let #optional_names = self.#optional_names.clone();)*
                    Ok(#original_id {
                        #(#all_names: #all_names,)*
                    })
                }
            }
        };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
