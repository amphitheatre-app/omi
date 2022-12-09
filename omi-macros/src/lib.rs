use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod entity;
mod creatable;
mod deletable;
mod queryable;
mod updatable;
mod util;

/// `#[dervie(Entity)]`
/// generate impl for trait Entity
/// 
/// Attributes:
/// 
/// `#[entity[table = "table_name"]]`
/// specify table name for Entity, default is the snake case of struct name
/// 
/// `#[column(primary)]`
/// primary key
/// 
/// `#[column(auto)]`
/// auto increament
/// 
/// `#[column(index)]`
/// index key
/// 
/// `#[column(type = "text"]`
/// specify column type, default is determined by the field type,
/// option value is `integer`, `float`, `text`, `blob`, `timestamp`, `date`, `time`, `datetime`, `year`, `boolean`
/// 
/// `#[column(rename = "column_name"]`
/// specify column name, default is the field name
/// 
/// `#[column(size = 255)`
/// the size of column type, only works for type of `integer`, `float`, `text`
/// 
/// `#[column(default = "default_value")]`
/// default value
/// 
/// `#[column(attrs(max_digits = 8, decimal_places = 2))]`
/// the decimal format of float type
/// 
/// field type with Option will make the column nullable
/// 
/// Example:
/// 
/// ```
/// #[derive(Debug, Entity, Queryable, PartialEq, Clone)]
/// #[entity(table = "products")]
/// struct Product {
///     #[column(primary, auto)]
///     id: u64,
/// 
///     #[column(size = 255, default = "", index)]
///     title: String,
///     
///     #[column(type = "float", attrs(max_digits = 8, decimal_places = 2))]
///     price: f32,
///
///     remark: Option<String>,
///
///     #[column(rename = "type")]
///     product_type: String,
///
///     #[column(default = "true")]
///     enabled: bool,
/// }
/// ```
/// 
#[proc_macro_derive(Entity, attributes(entity, column, relation))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    entity::impl_entity(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// `#[dervie(Creatable)]`
/// generate impl for trait Creatable
/// 
#[proc_macro_derive(Creatable)]
pub fn derive_creatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    creatable::impl_creatable(input)
        .into()
}

/// `#[dervie(Deletable)]`
/// generate impl for trait Deletable
/// 
#[proc_macro_derive(Deletable)]
pub fn derive_deletable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    deletable::impl_deletable(input)
        .into()
}

/// `#[dervie(Queryable)]`
/// generate impl for trait Queryable
/// 
#[proc_macro_derive(Queryable)]
pub fn derive_queryable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    queryable::impl_queryable(input)
        .into()
}

/// `#[dervie(Updatable)]`
/// generate impl for trait Updatable
/// 
#[proc_macro_derive(Updatable)]
pub fn derive_updatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    updatable::impl_updatable(input)
        .into()
}