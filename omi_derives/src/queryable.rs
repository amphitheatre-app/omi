use proc_macro2::TokenStream;

pub fn impl_queryable(input: syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;

    quote::quote! {
        impl omi::Queryable<T> for #struct_name {
            fn get() -> omi::Statement<T> {
                Statement::new(Opt::Select)
            }

            fn find() -> omi::Statement<T> {
                Statement::new(Opt::Select)
            }
        }
    }
}