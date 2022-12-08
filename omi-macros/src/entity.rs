use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use proc_macro_util::parse_keyed_strings;
use syn::spanned::Spanned;

pub struct DeriveEntity {
    ident: syn::Ident,
    table_name: String,
}

impl DeriveEntity {
    fn new(input: syn::DeriveInput) -> syn::Result<Self> {
        let ident = input.ident;

        let attr = input.attrs.iter().find(|a| a.path.is_ident("entity"));

        let table_name = if let Some(attr) = attr {
            let values =
                parse_keyed_strings(attr.tokens.clone().into_iter(), attr.span(), &["table"])
                    .map_err(|_| syn::Error::new(attr.span(), "expected `table` attribute"))?;
            let value = values.get("table").ok_or_else(|| {
                syn::Error::new(attr.span(), "expected `table` attribute with string value")
            })?;
            get_table_name(&ident, value)
        } else {
            get_table_name(&ident, "")
        };

        Ok(Self { ident, table_name })
    }

    fn expand(&self) -> TokenStream {
        self.impl_entity()
    }

    fn impl_entity(&self) -> TokenStream {
        let ident = &self.ident;
        let table_name = &self.table_name;

        quote::quote! {
            impl omi::model::Entity for #ident {
                fn meta() -> omi::model::Meta {
                    omi::model::Meta{
                        table: omi::model::Table {
                            name: String::from(#table_name),
                            ..Default::default()
                        },
                    }
                }
            }
        }
    }
}

fn get_table_name(struct_name: &syn::Ident, value: &str) -> String {
    if value.is_empty() {
        struct_name.to_string().to_snake_case()
    } else {
        value.to_owned().replace('"', "")
    }
}

pub fn impl_entity(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    Ok(DeriveEntity::new(input)?.expand())
}

#[test]
fn parse_entity_table_name_works_1() {
    let input = quote::quote! {
        struct MyEntity;
    };
    let derive_input = syn::parse2(input).unwrap();
    let derive_entity = DeriveEntity::new(derive_input).unwrap();

    assert_eq!("my_entity", derive_entity.table_name)
}

#[test]
fn parse_entity_table_name_works_2() {
    let input = quote::quote! {
        #[Entity(table = "my_entities")]
        struct MyEntity;
    };
    let derive_input = syn::parse2(input).unwrap();
    let derive_entity = DeriveEntity::new(derive_input).unwrap();

    assert_eq!("my_entities", derive_entity.table_name)
}
