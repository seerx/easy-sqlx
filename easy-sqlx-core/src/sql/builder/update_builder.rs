use crate::sql::{schema::table::TableSchema, utils::pair::Pair};



#[derive(Debug)]
pub struct UpdateBuilder<'a> {
    table: TableSchema,
    default_schema: &'a str,
    columns: Vec<Pair>,
}

impl<'a> UpdateBuilder<'a> {
    pub fn new(table: TableSchema) -> Self {
        Self {
            table,
            default_schema: "",
            columns: vec![],
        }
    }

    pub fn with_default_schema(mut self, schema: &'a str) -> Self {
        self.default_schema = schema;
        self
    }

    pub fn set_column(mut self, pair: Pair)-> Self {
        self.columns.push(pair);
        self
    }
}
