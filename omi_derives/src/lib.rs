use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod queryable;

#[proc_macro_derive(Queryable)]
pub fn derive_queryable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    queryable::impl_queryable(input)
        .into()
}