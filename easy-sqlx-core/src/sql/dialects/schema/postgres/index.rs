use std::io;

use regex::Regex;
use sqlx::{ColumnIndex, Database, Decode, Encode, Executor, IntoArguments, Type};

use crate::sql::{dialects::context::Context, schema::index::Index};

#[derive(Default, sqlx::FromRow, Debug)]
struct Idx {
    pub indexname: String,
    pub indexdef: String,
}

/// 获取索引列表
pub async fn get_indexes<'c, C, DB: Database>(
    context: &Context,
    conn: &mut C,
    table: &'c String,
    schema: Option<String>,
) -> io::Result<Vec<Index>>
where
    for<'e> &'e mut C: Executor<'e, Database = DB>,
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
    for<'a> String: Decode<'a, DB> + Type<DB>,
    for<'a> &'a str: ColumnIndex<DB::Row>,
    for<'a> std::string::String: Encode<'a, DB>,
{
    let schema_name = if let Some(s) = schema {
        s
    } else {
        context.get_default_schema()
    };

    // let sql = "SELECT indexname, indexdef FROM pg_indexes WHERE tablename=$1 {}".to_string();
    let query: Result<Vec<Idx>, sqlx::Error> = if schema_name.is_empty() {
        sqlx::query_as::<DB, Idx>("SELECT indexname, indexdef FROM pg_indexes WHERE tablename=$1")
            .bind(table)
            .fetch_all(&mut *conn)
            .await
    } else {
        sqlx::query_as::<DB, Idx>(
            "SELECT indexname, indexdef FROM pg_indexes WHERE tablename=$1 AND schemaname = $2",
        )
        .bind(table)
        .bind(schema_name)
        .fetch_all(&mut *conn)
        .await
    };

    let mut indexes = vec![];

    for idx in query.unwrap().iter() {
        if idx.indexname.to_lowercase() == "primary"
            || idx
                .indexname
                .to_lowercase()
                .as_bytes()
                .ends_with("_pkey".as_bytes())
        {
            // 主键，忽略
            continue;
        }

        let reg = Regex::new(r#"\((\s*\w+\s*[,\s*\w+\s*]*)\)"#).unwrap();
        if reg.is_match(&idx.indexdef) {
            for (_, [fields]) in reg.captures_iter(&idx.indexdef).map(|c| c.extract()) {
                let columns = fields
                    .split(",")
                    .map(|fd| fd.to_string())
                    .collect::<Vec<String>>();
                let index = Index {
                    name: idx.indexname.clone(),
                    unique: idx
                        .indexdef
                        .to_lowercase()
                        .as_bytes()
                        .starts_with("create unique index".as_bytes()),
                    columns: columns,
                };

                // aa.map(||)
                // println!("{:?}, -- {}", index, fields);
                indexes.push(index);
                break;
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("找不到索引 {} 的的字段", idx.indexname),
            ));
        }
    }

    Ok(indexes)
}
