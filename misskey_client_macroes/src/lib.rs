use proc_macro::{TokenStream};
use proc_macro2::Literal;
use quote::quote;
use syn::{Token, Type};
use syn::spanned::Spanned;

#[proc_macro_derive(FixedEndpointJsonRequest, attributes(misskey_client))]
pub fn derive_fixed_endpoint_json_request(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<syn::DeriveInput>(input).unwrap();
    let mut response: Option<Type> = None;
    let mut endpoint: Option<Literal> = None;
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
                        Err(_) => return Err(syn::Error::new(meta.value().map(|a| a.span()).unwrap_or(meta.input.span()), r#"Attributes must be following form: `misskey_client(endpoint = "endpoint", response = Response)`"#)),
                    }
                } else if meta.path.is_ident("response") && meta.input.peek(Token![=]) {
                    match meta.value().map(|a| a.parse::<Type>()).flatten() {
                        Ok(a) => {
                            if response.is_some() {
                                return Err(syn::Error::new(meta.path.span(), "Duplicated definition of response."));
                            }
                            response = Some(a)
                        },
                        Err(_) => return Err(syn::Error::new(meta.value().map(|a| a.span()).unwrap_or(meta.input.span()), r#"Attributes must be following form: `misskey_client(endpoint = "endpoint", response = Response)`"#)),
                    }
                } else {
                    return Err(syn::Error::new(meta.path.span(), r#"Attributes must be following form: `misskey_client(endpoint = "endpoint", response = Response)`"#));
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
    quote! {
        impl #generics #origin::FixedEndpointJsonRequest for #name #generics {
            type Response = #response;
            const ENDPOINT: &'static str = #endpoint;
        }
    }.into()
}
