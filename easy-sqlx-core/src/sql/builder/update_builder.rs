use crate::sql::{dialects::condition::Where, schema::table::TableSchema, utils::pair::Pair};

use super::builder::Builder;

#[derive(Debug)]
pub struct UpdateBuilder<'a> {
    table: TableSchema,
    default_schema: &'a str,
    columns: Vec<Pair>,
    wh: Option<Where>,
}

impl<'a> UpdateBuilder<'a> {
    pub fn new(table: TableSchema) -> Self {
        Self {
            table,
            default_schema: "",
            columns: vec![],
            wh: None,
        }
    }

    pub fn with_default_schema(mut self, schema: &'a str) -> Self {
        self.default_schema = schema;
        self
    }

    pub fn set_column(mut self, pair: Pair) -> Self {
        self.columns.push(pair);
        self
    }
}

#[cfg(feature = "postgres")]
use sqlx::Postgres;
impl<'a> Builder for UpdateBuilder<'a> {
    #[cfg(feature = "postgres")]
    type DB = Postgres;

    async fn execute<C>(
        &self,
        conn: &mut C,
    ) -> Result<<Self::DB as sqlx::Database>::QueryResult, sqlx::Error>
    where
        for<'e> &'e mut C: sqlx::Executor<'e, Database = Self::DB>,
    {
        todo!()
    }
}
