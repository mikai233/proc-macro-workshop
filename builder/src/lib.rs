use proc_macro::TokenStream;
use quote::quote;
use quote::{format_ident, ToTokens};
use syn::__private::TokenStream2;
use syn::parse_macro_input;
use syn::{Data, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let builder_name = format_ident!("{}Builder", struct_name);
    let builder = gen_builder(&input);
    let instance = gen_builder_instance(&input);
    let setters_fn = gen_setters_fn(&input);
    let build_fn = gen_build_fn(&input);
    let expanded = quote! {
        #builder
        impl #struct_name{
            pub fn builder() -> #builder_name{
                #instance
            }
        }
        impl #builder_name{
            #(#setters_fn)*
            #build_fn
        }
    };
    println!("{}", expanded);
    return expanded.into();
}
fn gen_builder(input: &DeriveInput) -> TokenStream2 {
    let struct_name = &input.ident;
    let builder_name = format_ident!("{}Builder", struct_name);
    let builder = match &input.data {
        Data::Struct(data) => {
            let fields_token = data
                .fields
                .iter()
                .map(|f| {
                    let field_name = f.ident.to_token_stream();
                    let field_type = &f.ty;
                    quote! {#field_name:Option<#field_type>}
                })
                .collect::<Vec<_>>();
            quote! {
                pub struct #builder_name {
                    #(#fields_token,)*
                }
            }
        }
        Data::Enum(_) => quote! {},
        Data::Union(_) => quote! {},
    };
    return builder;
}
fn gen_builder_instance(input: &DeriveInput) -> TokenStream2 {
    let struct_name = &input.ident;
    let builder_name = format_ident!("{}Builder", struct_name);
    let instance = match &input.data {
        Data::Struct(data) => {
            let fields_token = data
                .fields
                .iter()
                .map(|f| f.ident.to_token_stream())
                .collect::<Vec<_>>();
            quote! {
                #builder_name {
                    #(#fields_token: None,)*
                }
            }
        }
        Data::Enum(_) => quote! {},
        Data::Union(_) => quote! {},
    };
    return instance;
}
fn gen_setters_fn(input: &DeriveInput) -> Vec<TokenStream2> {
    let setters = match &input.data {
        Data::Struct(data) => data
            .fields
            .iter()
            .map(|f| {
                let field_name = f.ident.to_token_stream();
                quote! {
                    pub fn #field_name(&mut self, #f) -> &mut Self {
                        self.#field_name= Some(#field_name);
                        self
                    }
                }
            })
            .collect::<Vec<_>>(),
        Data::Enum(_) => vec![],
        Data::Union(_) => vec![],
    };
    return setters;
}

fn gen_build_fn(input: &DeriveInput) -> TokenStream2 {
    let struct_name = &input.ident;
    match &input.data {
        Data::Struct(data) => {
            let fields_tokens = data
                .fields
                .iter()
                .map(|f| {
                    let name = f.ident.to_token_stream();
                    quote! {#name: self.#name.as_ref().unwrap().to_owned(),}
                })
                .collect::<Vec<_>>();
            quote! {
                pub fn build(&mut self) -> Result<#struct_name, Box<dyn std::error::Error>> {
                    Ok(#struct_name{
                        #(#fields_tokens)*
                    })
                }
            }
        }
        Data::Enum(_) => quote! {},
        Data::Union(_) => quote! {},
    }
}
