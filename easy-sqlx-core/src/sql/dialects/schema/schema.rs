use sqlx::{ColumnIndex, Database, Decode, Encode, Executor, IntoArguments};

use crate::sql::{
    dialects::{
        condition::Where,
        page::{Order, PageRequest},
    },
    schema::{column::Column, index::Index, table::TableSchema},
    utils::quote::Quoter,
};
use std::{future::Future, io};

pub trait Schema
// where
//     for<'e> &'e mut C: Executor<'e, Database = DB>,
//     for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
{
    fn quoter(&self) -> Quoter;

    fn query_upgrade_tags<C, DB: Database>(
        &self,
        conn: &mut C,
        table_name: &String,
        tag: &String,
    ) -> impl Future<Output = io::Result<Vec<String>>> + Send
    where
        for<'a> &'a mut C: Executor<'a, Database = DB>,
        for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
        for<'a> &'a str: ColumnIndex<DB::Row>,
        for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB> + Encode<'a, DB>,
        for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>;

    // type DB: Database;
    // fn check_upgrade_table(&self, conn: &mut C) -> impl Future<Output = io::Result<()>> + Send;
    // fn query_upgrade_tags(
    //     &self,
    //     conn: &mut C,
    //     table_name: &String,
    //     tag: &String,
    // ) -> impl Future<Output = io::Result<Vec<String>>> + Send
    // where
    //     for<'e> &'e mut C: Executor<'e, Database = DB>,
    //     for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>;
    fn insert_upgrade_tag<C, DB: Database>(
        &self,
        conn: &mut C,
        table_name: &String,
        tag: &String,
        tag_value: &String,
    ) -> impl Future<Output = io::Result<()>> + Send
    where
        for<'a> &'a mut C: Executor<'a, Database = DB>,
        for<'a> <DB as Database>::Arguments<'a>: IntoArguments<'a, DB>,
        for<'a> &'a str: ColumnIndex<DB::Row>,
        for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB> + Encode<'a, DB>,
        for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>;

    fn table_name_with_schema(&self, table: &TableSchema) -> String;
    fn is_table_name_equal(&self, table1: &TableSchema, table2: &TableSchema) -> bool;

    fn execute_sql<'c, C, DB: Database>(
        &self,
        conn: &mut C,
        sql: &'c str,
    ) -> impl Future<Output = io::Result<<DB as Database>::QueryResult>> + Send
    where
        for<'a> &'a mut C: Executor<'a, Database = DB>,
        for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
        for<'a> &'a str: ColumnIndex<DB::Row>,
        for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB> + Encode<'a, DB>,
        for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>;

    // type Connection;
    fn get_tables<C, DB: Database>(
        &self,
        conn: &mut C,
    ) -> impl std::future::Future<Output = io::Result<Vec<TableSchema>>> + Send
    where
        for<'e> &'e mut C: Executor<'e, Database = DB>,
        for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
        for<'a> &'a str: ColumnIndex<DB::Row>,
        for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB> + Encode<'a, DB>,
        for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>;

    fn sql_create_table(&self, table: &TableSchema) -> io::Result<Vec<String>>;
    fn sql_drop_table(&self, table: &TableSchema) -> String;

    fn sql_create_index(&self, table: &TableSchema, index: &Index) -> Option<String>;
    fn sql_create_indexes(&self, table: &TableSchema) -> Vec<String>;
    fn sql_drop_index(&self, table: &TableSchema, index: &Index) -> String;

    fn sql_add_column(&self, table: &TableSchema, col: &Column) -> String;
    fn sql_alter_column(
        &self,
        table: &TableSchema,
        old: &Column,
        new: &Column,
    ) -> io::Result<Vec<String>>;
    fn sql_drop_column(&self, table: &TableSchema, col: &Column) -> String;

    fn sql_insert(&self, table: &TableSchema) -> String;
    fn sql_insert_columns(&self, table: &TableSchema, cols: &Vec<String>) -> String;
    fn sql_update_columns(
        &self,
        table: &TableSchema,
        cols: &Vec<String>,
        wh: Option<Where>,
    ) -> String;

    fn sql_delete(&self, table: &TableSchema, wh: Option<Where>) -> String;

    fn sql_select(
        &self,
        table: &TableSchema,
        wh: Option<Where>,
        orders: &Vec<Order>,
        pg: Option<&PageRequest>,
    ) -> String;
}
