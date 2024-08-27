use sqlx::{Database, Executor, IntoArguments};

use crate::sql::schema::{column::Column, index::Index, table::TableSchema};
use std::{future::Future, io};

pub trait Schema<'c, C, DB: Database>
where
    for<'e> &'e mut C: Executor<'e, Database = DB>,
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
{
    // type DB: Database;
    // fn check_upgrade_table(&self, conn: &mut C) -> impl Future<Output = io::Result<()>> + Send;
    fn query_upgrade_tags(
        &self,
        conn: &mut C,
        table_name: &String,
        tag: &String,
    ) -> impl Future<Output = io::Result<Vec<String>>> + Send;
    fn insert_upgrade_tag(
        &self,
        conn: &mut C,
        table_name: &String,
        tag: &String,
        tag_value: &String,
    ) -> impl Future<Output = io::Result<()>> + Send;

    fn table_name_with_schema(&self, table: &TableSchema) -> String;
    fn is_table_name_equal(&self, table1: &TableSchema, table2: &TableSchema) -> bool;

    fn execute_sql<'a>(
        &self,
        conn: &mut C,
        sql: &'a str,
    ) -> impl Future<Output = io::Result<DB::QueryResult>> + Send;

    // type Connection;
    fn get_tables(
        &self,
        conn: &mut C,
    ) -> impl std::future::Future<Output = io::Result<Vec<TableSchema>>> + Send;

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
}
