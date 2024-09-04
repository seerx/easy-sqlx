use crate::sql::{
    dialects::{
        condition::{Condition, Where, WhereAppend},
        schema::{self, schema::Schema},
    },
    schema::table::TableSchema,
};

use super::builder::QueryBuilder;
use sqlx::{Database, Execute as _};

#[derive(Debug)]
pub struct SelectBuilder<'a> {
    table: TableSchema,
    default_schema: &'a str,
    columns: Vec<String>,
    wh: Option<Where>,
}

impl<'a> SelectBuilder<'a> {
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

    pub fn column(mut self, col: String) -> Self {
        self.columns.push(col);
        self
    }

    pub fn columns(mut self, cols: Vec<String>) -> Self {
        self.columns.extend(cols);
        self
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
    
    fn fetch<'e, 'c: 'e, E>(self, executor: E) -> futures::stream::BoxStream<'e, Result<O, sqlx::Error>>
    where
        E: 'e + sqlx::Executor<'c, Database = Self::DB>,
        O: 'e {
            // let schema = schema::new::<E, Self::DB>(self.default_schema.to_string());
        todo!()
    }

     

    // fn fetch<'e, 'c: 'e, E>(
    //     self,
    //     executor: E,
    // ) -> futures::stream::BoxStream<'e, Result<<Self::DB as Database>::Row, sqlx::Error>>
    // where
    //     'a: 'e,
    //     E: sqlx::Executor<'c, Database = Self::DB>,
    // {
    //     let schema = schema::new::<E, Self::DB>(self.default_schema.to_string());

    //     let sql = schema.sql_select(&self.table, &self.columns, self.wh.clone());

    //     let mut query: sqlx::query::Query<'_, Self::DB, <Self::DB as Database>::Arguments<'_>> =
    //         sqlx::query::<Self::DB>(&sql.to);

    //     if let Some(w) = &self.wh {
    //         query = w.bind_to_query(query);
    //     }

    //     tracing::debug!("easy-sqlx: {}", query.sql());
    //     query
    // }

    // async fn execute<C>(
    //     &self,
    //     conn: &mut C,
    // ) -> Result<sqlx::query::Query<'a, Postgres, PgArguments>, sqlx::Error>
    // where
    //     for<'e> &'e mut C: sqlx::Executor<'e, Database = Self::DB>,
    // {

    //     query.f
    //     query.execute(conn).await
    // }

    // fn query<C>(
    //     &self,
    //     conn: &mut C,
    // ) -> sqlx::query::Query<'a, Self::DB, <Self::DB as Database>::Arguments<'a>>
    // where
    //     for<'e> &'e mut C: sqlx::Executor<'e, Database = Self::DB>,
    // {
    //     let schema = schema::new::<C, Self::DB>(self.default_schema.to_string());

    //     let sql = schema.sql_select(&self.table, &self.columns, self.wh.clone());

    //     let mut query: sqlx::query::Query<'_, Self::DB, <Self::DB as Database>::Arguments<'_>> =
    //         sqlx::query::<Self::DB>(&sql.to);

    //     if let Some(w) = &self.wh {
    //         query = w.bind_to_query(query);
    //     }

    //     tracing::debug!("easy-sqlx: {}", query.sql());
    //     query
    // }
}
