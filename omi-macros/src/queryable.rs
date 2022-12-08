use proc_macro2::TokenStream;

pub fn impl_queryable(input: syn::DeriveInput) -> TokenStream {
    let ident = &input.ident;

    quote::quote! {
        impl omi::operations::Queryable<#ident> for #ident {
            fn find() -> omi::statement::SelectStatement<#ident> {
                omi::statement::SelectStatement::new()
            }
        }
    }
}
