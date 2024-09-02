use sqlx::{Database, Execute, Executor, Postgres};

use crate::sql::{
    dialects::schema::{self, schema::Schema as _},
    schema::table::TableSchema,
    utils::pair::Pair,
};

use super::builder::Builder;

#[derive(Debug)]
pub struct InsertBuilder<'a> {
    table: TableSchema,
    default_schema: &'a str,
    columns: Vec<Pair>,
}

impl<'a> InsertBuilder<'a> {
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

    pub fn add_column(&mut self, pair: Pair) {
        self.columns.push(pair);
    }
}

impl<'a> Builder for InsertBuilder<'a>
// <DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, DB>,
{
    #[cfg(feature = "postgres")]
    type DB = Postgres;

    async fn execute<C>(
        &self,
        conn: &mut C,
    ) -> Result<<Self::DB as Database>::QueryResult, sqlx::Error>
    where
        for<'e> &'e mut C: Executor<'e, Database = Self::DB>,
    {
        let schema = schema::new::<C, Self::DB>(self.default_schema.to_string());
        let cols: Vec<String> = self.columns.iter().map(|c| c.name.to_string()).collect();
        let sql = schema.sql_insert_columns(&self.table, &cols);

        let mut query: sqlx::query::Query<'_, Self::DB, <Self::DB as Database>::Arguments<'_>> =
            sqlx::query::<Self::DB>(&sql);

        for col in &self.columns {
            query = col.bind_to_query(query);
        }

        tracing::debug!("easy-sqlx: {}", query.sql());

        query.execute(conn).await
    }
}
