use proc_macro2::TokenStream;

pub fn impl_deletable(input: syn::DeriveInput) -> TokenStream {
    let ident = &input.ident;

    quote::quote! {
        impl omi::operations::Deletable<#ident> for #ident {
            fn delete(entity: #ident) -> omi::statement::DeleteStatement<#ident> {
                omi::statement::DeleteStatement::new(entity)
            }
        }
    }
}
