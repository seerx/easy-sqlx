pub(crate) mod postgres;

pub mod schema;

use super::context::Context;

#[cfg(feature = "postgres")]
use postgres::schema::PgSchema;
use schema::Schema; 
// use sqlx::{Database, Executor, Postgres};

pub fn new(default_sechma: String) -> impl Schema
// impl Schema<Postgres, T>
// where
//     for<'e> &'e mut C: Executor<'e, Database = DB>,
//     for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
//     for<'a> &'a str: ColumnIndex<DB::Row>,
//     for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
//     for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
//     for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
//     for<'a> i64: Encode<'a, DB>,
//     for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>,
{
    #[cfg(feature = "postgres")]
    let schema = PgSchema::new(Context::new(default_sechma));
    schema
}

// Box<dyn schema::Schema<Connection = PgConnection>>
