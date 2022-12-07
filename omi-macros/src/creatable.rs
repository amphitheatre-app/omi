use proc_macro2::TokenStream;

pub fn impl_creatable(input: syn::DeriveInput) -> TokenStream {
    let ident = &input.ident;

    quote::quote! {
        impl omi::operations::Creatable<#ident> for #ident {
            fn create(entity: #ident) -> omi::Statement<#ident> {
                omi::Statement::new(omi::Ops::Insert)
            }
        }
    }
}