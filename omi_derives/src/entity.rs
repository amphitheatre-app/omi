use proc_macro2::TokenStream;

pub fn impl_entity(input: syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;

    quote::quote! {
        impl omi::Entity for #struct_name {
            fn meta(&self) -> omi::Meta {
                omi::Meta{
                    table: stringify!(#struct_name).to_lowercase(),
                }
            }
        }
    }
}