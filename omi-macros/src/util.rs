use proc_macro2::TokenStream;
use syn::{PathArguments, AngleBracketedGenericArguments, GenericArgument};

pub fn get_type_path(ty: &syn::Type) -> Option<syn::Path> {
    if let syn::Type::Path(path) = ty {
        Some(path.clone().path)
    } else {
        None
    }
}

pub fn is_option_type(type_path: &syn::Path) -> bool {
    if let Some(segment) = type_path.segments.first() {
        segment.ident == "Option"
    } else {
        false
    }
}

pub fn get_inner_type_path(type_path: &syn::Path) -> Option<syn::Path> {
    if let Some(segment) = type_path.segments.first() {
        if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {ref args , ..}) = segment.arguments {
            if let Some(GenericArgument::Type(ty)) = args.first() {
                return get_type_path(ty)
            }
        }
    }
    None
}

pub fn quote_int_option<T: quote::ToTokens>(val: &Option<T>) -> TokenStream {
    if let Some(val) = val {
        quote::quote!(Some(#val))
    } else {
        quote::quote!(None)
    }
}

pub fn quote_str_option<T: quote::ToTokens>(val: &Option<T>) -> TokenStream {
    if let Some(val) = val {
        quote::quote!(Some(String::from(#val)))
    } else {
        quote::quote!(None)
    }
}