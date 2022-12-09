use darling::{FromDeriveInput, FromField, FromMeta};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use syn::{spanned::Spanned};

use crate::util::{get_type_path, is_option_type, get_inner_type_path, quote_str_option, quote_int_option};

#[derive(Debug, Clone, FromMeta)]
struct ColumnAttr {
    max_digits: Option<usize>,

    decimal_places: Option<usize>,
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(column))]
struct EntityField {
    ident: Option<syn::Ident>,

    ty: syn::Type,

    rename: Option<String>,

    #[darling(rename = "type")]
    column_type: Option<String>,

    #[darling(default)]
    primary: bool,

    size: Option<usize>,

    default: Option<String>,

    #[darling(default)]
    auto: bool,

    #[darling(default)]
    unique: bool,

    #[darling(default)]
    index: bool,

    #[darling(rename = "attrs")]
    attr: Option<ColumnAttr>,
}

impl EntityField {
    fn get_ident(&self) -> syn::Ident {
        self.ident.clone().expect("should have ident")
    }

    fn get_conlumn_name(&self) -> String {
        match self.rename {
            Some(ref rename) if !rename.is_empty() => {
                rename.to_owned()
            }
            _ => {
                self.get_ident().to_string().to_snake_case()
            }
        }
    }

    fn get_column_kind(&self) -> syn::Result<(bool, TokenStream)> {
        let mut null = false;
        let gen: TokenStream = if let Some(column_type) = self.column_type.clone() {
            match column_type.as_str() {
                "integer" => {
                    let size = quote_int_option(&self.size);
                    quote::quote!(omi::model::DataKind::Integer(#size))
                },
                "float" => {
                    let max_digits = quote_int_option(&self.attr.clone().and_then(|a| a.max_digits));
                    let decimal_places = quote_int_option(&self.attr.clone().and_then(|a| a.decimal_places));
                    quote::quote!(omi::model::DataKind::Float{
                        max_digits: #max_digits,
                        decimal_places: #decimal_places,
                    })
                },
                "text" => {
                    let size = quote_int_option(&self.size);
                    quote::quote!(omi::model::DataKind::Text(#size))
                },
                "blob" => quote::quote!(omi::model::DataKind::Blob),
                "timestamp" => quote::quote!(omi::model::DataKind::Timestamp),
                "date" => quote::quote!(omi::model::DataKind::Date),
                "time" => quote::quote!(omi::model::DataKind::Time),
                "datetime" => quote::quote!(omi::model::DataKind::Datetime),
                "year" => quote::quote!(omi::model::DataKind::Year),
                "boolean" => quote::quote!(omi::model::DataKind::Boolean),
                _ => {
                    return Err(syn::Error::new(self.get_ident().span(), "invalid type for `column`"));
                }
            }
        } else if let Some(mut type_path) = get_type_path(&self.ty) {
            if is_option_type(&type_path) {
                null = true;
                type_path = get_inner_type_path(&type_path).ok_or_else(|| syn::Error::new(self.get_ident().span(), "unsupport field type"))?;
            }

            if let Some(ident) = type_path.get_ident() {
                match ident.to_string().as_str() {
                    "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "isize" | "usize" => {
                        let size = quote_int_option(&self.size);
                        quote::quote!(omi::model::DataKind::Integer(#size))
                    }
                    "f32" | "f64" => {
                        let max_digits = quote_int_option(&self.attr.clone().and_then(|a| a.max_digits));
                        let decimal_places = quote_int_option(&self.attr.clone().and_then(|a| a.decimal_places));
                        quote::quote!(omi::model::DataKind::Float{
                            max_digits: #max_digits,
                            decimal_places: #decimal_places,
                        })
                    }
                    "String" => {
                        let size = quote_int_option(&self.size);
                        quote::quote!(omi::model::DataKind::Text(#size))
                    }
                    "bool" => {
                        quote::quote!(omi::model::DataKind::Boolean)
                    }
                    _ => {
                        return Err(syn::Error::new(self.get_ident().span(), "unsupport field type"));
                    }
                }
            } else {
                return Err(syn::Error::new(self.get_ident().span(), "unsupport field type"));
            }
        } else {
            return Err(syn::Error::new(self.get_ident().span(), "unsupport field type"));
        };

        Ok((null, gen))
    }

}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(entity))]
struct DeriveEntity {
    ident: syn::Ident,

    table: Option<String>,

    data: darling::ast::Data<darling::util::Ignored, EntityField>,

    #[darling(skip)]
    fields: Vec<EntityField>
}

impl DeriveEntity {
    fn new(input: syn::DeriveInput) -> syn::Result<Self> {
        let mut derive_entity = Self::from_derive_input(&input).map_err(|e| syn::Error::new(input.span(), e.to_string()))?;
        derive_entity.fields = derive_entity.data.clone()
            .take_struct()
            .ok_or_else(|| syn::Error::new(derive_entity.ident.span(), ""))?
            .fields;

        Ok(derive_entity)
    }

    fn expand(&self) -> syn::Result<TokenStream> {
        self.impl_entity()
    }

    fn impl_entity(&self) -> syn::Result<TokenStream> {
        let ident = &self.ident;
        let table_name = &self.get_table_name();

        let conlumns = &self.expand_columns()?;

        let gen = quote::quote! {
            impl omi::model::Entity for #ident {
                fn meta() -> omi::model::Meta {
                    omi::model::Meta{
                        table: omi::model::Table {
                            name: String::from(#table_name),
                            columns: #conlumns,
                        },
                    }
                }
            }
        };

        Ok(gen)
    }

    fn get_table_name(&self) -> String {
        match &self.table {
            Some(name) => name.to_owned(),
            _ => self.ident.to_string().to_snake_case()
        }
    }

    fn expand_columns(&self) -> syn::Result<TokenStream> {
        let mut tokens = Vec::new();
        
        for field in self.fields.iter() {
            let token = Self::expand_colunn(field)?;
            tokens.push(token);
        }
    
        let gen = quote::quote! {
            vec![#(#tokens),*]
        };

        Ok(gen)
    }

    fn expand_colunn(field: &EntityField) -> syn::Result<TokenStream> {
        let name = field.get_conlumn_name();
        let (null, column_kind) = field.get_column_kind()?;
        let default = quote_str_option(&field.default);
        let primary = field.primary;
        let auto = field.auto;
        let unique = field.unique;
        let index = field.index;

        let gen = quote::quote! {
            omi::model::Column{
                name: String::from(#name),
                kind: #column_kind,
                null: #null,
                primary: #primary,
                auto: #auto,
                unique: #unique,
                index: #index,
                default: #default,
            }
        };
    
        Ok(gen)
    }
    
}

pub fn impl_entity(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    DeriveEntity::new(input)?.expand()
}

#[test]
fn test() {
    let input = quote::quote! {
        #[entity(table = "my_entities")]
        struct MyEntity{
            id: i64,
            remark: Option<String>
        }
    };
    let derive_input: syn::DeriveInput = syn::parse2(input).unwrap();

    let entity = DeriveEntity::new(derive_input).unwrap();

    entity.expand_columns().unwrap();
    
}