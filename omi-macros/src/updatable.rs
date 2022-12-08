use proc_macro2::TokenStream;

pub fn impl_updatable(input: syn::DeriveInput) -> TokenStream {
    let ident = &input.ident;

    quote::quote! {
        impl omi::operations::Updatable<#ident> for #ident {
            fn update(entity: Option<#ident>) -> omi::statement::UpdateStatement<#ident> {
                omi::statement::UpdateStatement::new(entity)
            }
        }
    }
}
