use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{LitBool, Token, Type};
use syn::spanned::Spanned;

#[proc_macro_derive(ConstParamJsonRequest, attributes(misskey_client))]
pub fn derive_const_param_json_request(input: TokenStream) -> TokenStream {
    const ERR_MESSAGE: &str = r#"Attributes must be following form: `misskey_client(endpoint = "endpoint", response = Response, can_be_empty = bool)`"#;
    let ast = syn::parse::<syn::DeriveInput>(input).unwrap();
    let mut response: Option<Type> = None;
    let mut endpoint: Option<Literal> = None;
    let mut can_be_empty: Option<LitBool> = None;
    let origin = match proc_macro_crate::crate_name("misskey_client").unwrap() {
        proc_macro_crate::FoundCrate::Itself => quote! {crate},
        proc_macro_crate::FoundCrate::Name(_) => quote! {misskey_client},
    };
    for attr in &ast.attrs {
        if attr.path().is_ident("misskey_client") {
            if let Err(e) = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("endpoint") && meta.input.peek(Token![=]) {
                    match meta.value().map(|a| a.parse::<Literal>()).flatten() {
                        Ok(a) => {
                            if endpoint.is_some() {
                                return Err(syn::Error::new(meta.path.span(), "Duplicated definition of endpoint."));
                            }
                            endpoint = Some(a)
                        },
                        Err(_) => return Err(syn::Error::new(meta.value().map(|a| a.span()).unwrap_or(meta.input.span()), ERR_MESSAGE)),
                    }
                } else if meta.path.is_ident("response") && meta.input.peek(Token![=]) {
                    match meta.value().map(|a| a.parse::<Type>()).flatten() {
                        Ok(a) => {
                            if response.is_some() {
                                return Err(syn::Error::new(meta.path.span(), "Duplicated definition of response."));
                            }
                            response = Some(a)
                        },
                        Err(_) => return Err(syn::Error::new(meta.value().map(|a| a.span()).unwrap_or(meta.input.span()), ERR_MESSAGE)),
                    }
                } else if meta.path.is_ident("can_be_empty") && meta.input.peek(Token![=]) {
                    match meta.value().map(|a| a.parse::<LitBool>()).flatten() {
                        Ok(a) => {
                            if can_be_empty.is_some() {
                                return Err(syn::Error::new(meta.path.span(), "Duplicated definition of can_be_empty."));
                            }
                            can_be_empty = Some(a)
                        },
                        Err(e) => return Err(syn::Error::new(meta.value().map(|a| a.span()).unwrap_or(meta.input.span()), e.to_string())),
                    }
                } else {
                    return Err(syn::Error::new(meta.path.span(), ERR_MESSAGE));
                }
                Ok(())
            }) {
                return e.into_compile_error().into();
            }
        }
    }
    let Some(response) = response else {
        return syn::Error::new(ast.span(), "Missing response type.").into_compile_error().into();
    };
    let Some(endpoint) = endpoint else {
        return syn::Error::new(ast.span(), "Missing endpoint.").into_compile_error().into();
    };
    let generics = &ast.generics;
    let name = &ast.ident;
    match can_be_empty {
        Some(can_be_empty) => quote! {
            impl #generics #origin::ConstParamJsonRequest for #name #generics {
                type Response = #response;
                const ENDPOINT: &'static str = #endpoint;
                const CAN_BE_EMPTY: bool = #can_be_empty;
            }
        },
        None => quote! {
            impl #generics #origin::ConstParamJsonRequest for #name #generics {
                type Response = #response;
                const ENDPOINT: &'static str = #endpoint;
            }
        },
    }.into()
}
