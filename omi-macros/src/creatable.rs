use proc_macro2::TokenStream;

pub fn impl_creatable(input: syn::DeriveInput) -> TokenStream {
    let ident = &input.ident;

    quote::quote! {
        impl omi::operations::Creatable<#ident> for #ident {
            fn create(entity: #ident) -> omi::statement::InsertStatement<#ident> {
                omi::statement::InsertStatement::new(entity)
            }
        }
    }
}
