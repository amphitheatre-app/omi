use super::Table;

#[derive(Debug, Default)]
pub struct Meta {
    pub table: Table,
}

#[cfg(test)]
mod test {
    use omi::prelude::*;

    use crate as omi;

    #[derive(Debug, Default, Clone, Entity)]
    #[entity(table = "products")]
    struct Product {}

    #[test]
    fn test_entity_meta() {
        assert_eq!(Product::meta().table.name, "products");
    }
}
