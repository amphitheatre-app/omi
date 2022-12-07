use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod entity;

#[proc_macro_derive(Entity, attributes(entity, column, relation))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    entity::impl_entity(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
