use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod entity;
mod creatable;
mod deletable;
mod queryable;
mod updatable;

#[proc_macro_derive(Entity, attributes(entity, column, relation))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    entity::impl_entity(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Creatable)]
pub fn derive_creatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    creatable::impl_creatable(input)
        .into()
}

#[proc_macro_derive(Deletable)]
pub fn derive_deletable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    deletable::impl_deletable(input)
        .into()
}

#[proc_macro_derive(Queryable)]
pub fn derive_queryable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    queryable::impl_queryable(input)
        .into()
}

#[proc_macro_derive(Updatable)]
pub fn derive_updatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    updatable::impl_updatable(input)
        .into()
}
