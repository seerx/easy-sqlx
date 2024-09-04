use crate::sql::{
    dialects::{
        condition::{Condition, Where, WhereAppend},
        schema::{self, schema::Schema},
    },
    schema::table::TableSchema,
};

use super::builder::ExecuteBuilder;
use sqlx::{Database, Execute as _};

#[derive(Debug)]
pub struct DeleteBuilder<'a> {
    table: TableSchema,
    default_schema: &'a str,
    wh: Option<Where>,
}

impl<'a> DeleteBuilder<'a> {
    pub fn new(table: TableSchema) -> Self {
        Self {
            table,
            default_schema: "",
            wh: None,
        }
    }

    pub fn with_default_schema(mut self, schema: &'a str) -> Self {
        self.default_schema = schema;
        self
    }
}
impl<'a> WhereAppend<Condition> for DeleteBuilder<'a> {
    fn and(mut self, cond: Condition) -> Self {
        if let Some(w) = self.wh {
            self.wh = Some(w.and(cond));
        } else {
            self.wh = Some(Where::new(cond));
        }
        self
    }

    fn or(mut self, cond: Condition) -> Self {
        if let Some(w) = self.wh {
            self.wh = Some(w.or(cond));
        } else {
            self.wh = Some(Where::new(cond));
        }
        self
    }
}

impl<'a> WhereAppend<Where> for DeleteBuilder<'a> {
    fn and(mut self, wh: Where) -> Self {
        if let Some(w) = self.wh {
            self.wh = Some(w.and(wh));
        } else {
            self.wh = Some(wh);
        }
        self
    }

    fn or(mut self, wh: Where) -> Self {
        if let Some(w) = self.wh {
            self.wh = Some(w.or(wh));
        } else {
            self.wh = Some(wh);
        }
        self
    }
}

#[cfg(feature = "postgres")]
use sqlx::Postgres;

impl<'a> ExecuteBuilder for DeleteBuilder<'a> {
    #[cfg(feature = "postgres")]
    type DB = Postgres;

    async fn execute<C>(
        &self,
        conn: &mut C,
    ) -> Result<<Self::DB as sqlx::Database>::QueryResult, sqlx::Error>
    where
        for<'e> &'e mut C: sqlx::Executor<'e, Database = Self::DB>,
    {
        let schema = schema::new(self.default_schema.to_string());

        let sql = schema.sql_delete(&self.table, self.wh.clone());

        let mut query: sqlx::query::Query<'_, Self::DB, <Self::DB as Database>::Arguments<'_>> =
            sqlx::query::<Self::DB>(&sql);

        if let Some(w) = &self.wh {
            query = w.bind_to_query(query);
        }

        tracing::debug!("easy-sqlx: {}", query.sql());

        query.execute(conn).await
    }
}
