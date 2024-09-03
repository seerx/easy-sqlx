use crate::sql::{
    dialects::{condition::{Condition, Where, WhereAppend}, schema::{self, schema::Schema}},
    schema::table::TableSchema,
    utils::pair::Pair,
};

use sqlx::{Database, Execute as _};
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
        // self.r#where()
        self
    }
}
impl<'a> WhereAppend<Condition> for UpdateBuilder<'a> {
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

impl<'a> WhereAppend<Where> for UpdateBuilder<'a> {
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
        let schema = schema::new::<C, Self::DB>(self.default_schema.to_string());
        
        let cols: Vec<String> = self.columns.iter().map(|c| c.name.to_string()).collect();
        let mut sql = schema.sql_update_columns(&self.table, &cols);
        if let Some(w) = &self.wh {
            let (ws, _) = w.sql(self.columns.len() + 1, &schema.quoter());
            if !ws.is_empty() { 
                sql.push_str(" where ");
                sql.push_str(&ws);
            }
        }

        // tracing::info!("easy-sqlx: {}", &sql);
        // let w_sql = self.wh
        let mut query: sqlx::query::Query<'_, Self::DB, <Self::DB as Database>::Arguments<'_>> =
            sqlx::query::<Self::DB>(&sql);

        for col in &self.columns {
            query = col.bind_to_query(query);
        }
        
        if let Some(w) = &self.wh {
            query = w.bind_to_query(query);
        }

        tracing::debug!("easy-sqlx: {}", query.sql());

        query.execute(conn).await
    }
}
