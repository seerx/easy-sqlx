use std::io;

use sqlx::{ColumnIndex, Database, Decode, Encode, Executor, IntoArguments};

use crate::sql::{
    dialects::{
        self,
        schema::schema::{self, Schema},
    },
    schema::table::TableSchema,
};

pub mod table;

// pub async fn execute<'a, DB: Database, T>(self, executor: T) -> Result<DB::QueryResult, Error>
// where
// for<'e> &'e mut T: Executor<'e, Database = Sqlite>,
//     E: Executor<'a, Database = DB>,
//     <DB as HasArguments<'a>>::Arguments: IntoArguments<'a, DB>,
// {
//     sqlx::query("").execute(executor).await
// }

pub async fn sync_tables<C, DB: Database>(
    conn: &mut C,
    tables: Vec<TableSchema>,
    default_schema: &str,
) -> io::Result<()>
// where
//     for<'e> &'e mut T: Executor<'e, Database = Postgres>,
where
    for<'e> &'e mut C: Executor<'e, Database = DB>,
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
    for<'a> &'a str: ColumnIndex<DB::Row>,
    for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
    for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
    for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
    for<'a> i64: Encode<'a, DB>,
    for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>,
{
    // 查询数据库中表
    let s = &dialects::schema::new(default_schema.to_string());

    // 删除含有 recreate 控制字段的表
    check_recreate(&tables, &mut *conn, s).await?;

    let db_tables = s.get_tables(&mut *conn).await?;
    // 遍历程序中定义的表
    for table in tables {
        if let Some(db_table) = db_tables.iter().find(|t| s.is_table_name_equal(&table, t)) {
            // 数据库中已经存在此表，检查字段差异
            for col in &table.columns {
                if let Some(db_col) = db_table.columns.iter().find(|c| col.is_name_equal(&c)) {
                    // 列存在，检查列差异
                    let sqls = s.sql_alter_column(&table, &db_col, &col)?;
                    for sql in sqls {
                        s.execute_sql(&mut *conn, &sql).await?;
                        // println!("column: {sql}");
                        // println!("column: {:?}", db_col);
                    }
                } else {
                    // 列不存在，添加列
                    let sql = s.sql_add_column(db_table, &col);
                    s.execute_sql(&mut *conn, &sql).await?;
                }
            }
            if table.trim_columns {
                // 清理未定义的列
                for db_col in &db_table.columns {
                    if !table.columns.iter().any(|c| c.is_name_equal(db_col)) {
                        // 定义中没有该列，删除数据库中的列
                        let sql = s.sql_drop_column(&table, db_col);
                        s.execute_sql(&mut *conn, &sql).await?;
                    }
                }
            }

            // 检查索引变化
            if let Some(new_indexes) = &table.indexes {
                for index in new_indexes {
                    if let Some(olds) = &db_table.indexes {
                        // 存在旧索引
                        // 查找旧索引
                        if let Some(old) = olds.iter().find(|idx| idx.is_name_equal(index)) {
                            //  检查索引是否发生变化
                            if old.is_columns_equal(index) {
                                // 列没有发生变化
                                continue;
                            }
                            // 索引列发生变化
                            // 删除旧索引
                            let sql = s.sql_drop_index(db_table, old);
                            s.execute_sql(&mut *conn, &sql).await?;
                        }
                    }
                    // 没有旧索引，创建索引
                    if let Some(sql) = &s.sql_create_index(&table, &index) {
                        s.execute_sql(&mut *conn, &sql).await?;
                    }
                }
            }
            if table.trim_indexes {
                // 清理未定义的索引
                if let Some(old) = &db_table.indexes {
                    for oidx in old {
                        if let Some(idxs) = &table.indexes {
                            if idxs.iter().any(|idx| idx.is_name_equal(oidx)) {
                                // 该索引已定义，不要删除
                                continue;
                            }
                        }
                        // 删除索引
                        let sql = s.sql_drop_index(db_table, oidx);
                        s.execute_sql(&mut *conn, &sql).await?;
                    }
                }
            }
        } else {
            // 数据库中不存在此表，创建 table
            let table_sqls = s.sql_create_table(&table)?;
            for sql in table_sqls.iter() {
                s.execute_sql(&mut *conn, &sql).await?;
            }

            // 创建索引
            let index_sqls = s.sql_create_indexes(&table);
            for sql in index_sqls.iter() {
                s.execute_sql(&mut *conn, &sql).await?;
            }
        }
    }

    Ok(())
}

async fn check_recreate<'c, T, DB: Database>(
    tables: &Vec<TableSchema>,
    conn: &mut T,
    s: &impl schema::Schema,
) -> io::Result<()>
where
    for<'e> &'e mut T: Executor<'e, Database = DB>,
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
    for<'a> &'a str: ColumnIndex<DB::Row>,
    for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
    for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
    for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
    for<'a> i64: Encode<'a, DB>,
    for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>,
{
    for table in tables {
        if let Some(value) = &table.recreate {
            // 有 recreate 定义
            let table_name = &s.table_name_with_schema(table);
            // tracing::info!("recreate {table_name}");
            let values = s
                .query_upgrade_tags(&mut *conn, table_name, &"recreate".to_string())
                .await?;
            // tracing::info!("join -----");
            let found = values.iter().any(|v| v == value);
            if !found {
                // 删除表
                let sql = s.sql_drop_table(table);
                // tracing::info!("删除表 ----- {sql}");
                s.execute_sql(&mut *conn, &sql).await?;
                // tracing::info!("删除表{table_name}");
                // 添加记录
                s.insert_upgrade_tag(&mut *conn, table_name, &"recreate".to_string(), value)
                    .await?;
            }
        }
    }
    Ok(())
}
