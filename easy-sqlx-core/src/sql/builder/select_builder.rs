use crate::sql::{
    dialects::{
        condition::{Condition, Where, WhereAppend},
        schema::{self, schema::Schema},
    },
    schema::table::TableSchema,
};

use super::builder::QueryBuilder;
use sqlx::{Database, FromRow};

#[derive(Debug)]
pub struct SelectBuilder<'a> {
    table: TableSchema,
    default_schema: &'a str,
    wh: Option<Where>,
}

impl<'a> SelectBuilder<'a> {
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

    fn generate_query_as(&self) -> String {
        let schema = schema::new(self.default_schema.to_string());
        let sql = schema.sql_select(&self.table, self.wh.clone());
        sql
    }
}
impl<'a> WhereAppend<Condition> for SelectBuilder<'a> {
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

impl<'a> WhereAppend<Where> for SelectBuilder<'a> {
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

impl<'a, O> QueryBuilder<'a, O> for SelectBuilder<'a> {
    #[cfg(feature = "postgres")]
    type DB = Postgres;

    async fn fetch_one<'e, 'c: 'e, E>(self, executor: E) -> Result<O, sqlx::Error>
    where
        E: 'e + sqlx::Executor<'c, Database = Self::DB>,
        O: 'e,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
        O: std::marker::Send,
        O: Unpin,
    {
        let sql = self.generate_query_as();
        let mut query = sqlx::query_as::<Self::DB, O>(&sql);
        if let Some(w) = &self.wh {
            query = w.bind_to_query_as(query);
        }

        let result = query.fetch_one(executor).await?;

        Ok(result)

        // todo!()
    }

    async fn fetch_optional<'e, 'c: 'e, E>(self, executor: E) -> Result<Option<O>, sqlx::Error>
    where
        E: 'e + sqlx::Executor<'c, Database = Self::DB>,
        O: 'e,
        O: std::marker::Send,
        O: Unpin,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
    {
        let sql = self.generate_query_as();
        let mut query = sqlx::query_as(&sql);
        if let Some(w) = &self.wh {
            query = w.bind_to_query_as(query);
        }

        let result = query.fetch_optional(executor).await?;

        Ok(result)
    }

    async fn fetch_all<'e, 'c: 'e, E>(self, executor: E) -> Result<Vec<O>, sqlx::Error>
    where
        E: 'e + sqlx::Executor<'c, Database = Self::DB>,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
        O: 'e,
        O: std::marker::Send,
        O: Unpin,
    {
        let sql = self.generate_query_as();
        let mut query = sqlx::query_as(&sql);
        if let Some(w) = &self.wh {
            query = w.bind_to_query_as(query);
        }

        let result = query.fetch_all(executor).await?;

        Ok(result)
    }
}
