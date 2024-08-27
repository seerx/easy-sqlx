use std::io;

use sqlx::{ColumnIndex, Database, Decode, Encode, Executor, IntoArguments, Type};

use crate::sql::{
    dialects::context::Context,
    schema::{
        column::Column,
        types::{match_sql_type, types::SqlType},
    },
};

#[derive(Default, sqlx::FromRow, Debug)]
struct Col {
    pub column_name: String,
    pub column_default: Option<String>,
    pub is_nullable: String,
    pub data_type: String,
    pub character_maximum_length: Option<i32>,
    pub primarykey: bool,
    // pub uniquekey: bool,
}

/// 判断自增类型
/// const AUTO_INCR_VALUE: &str = "nextval('pt_account_test_seq'::regclass)";
fn is_autoincr(default_value: &String) -> bool {
    default_value
        .to_lowercase()
        .as_bytes()
        .starts_with("nextval(".as_bytes())
        && default_value
            .to_lowercase()
            .as_bytes()
            .ends_with("::regclass)".as_bytes())
}

impl Col {
    pub fn to_column(&self) -> Column {
        // let mut data_len = None;
        let sql_type = match match_sql_type(&self.data_type) {
            Ok(name) => {
                if self.character_maximum_length.is_some() {
                    SqlType {
                        name: name.to_string(),
                        len: Some(self.character_maximum_length.unwrap() as isize),
                        len2: None,
                        fixed_len: if self.character_maximum_length.unwrap() == 1 {
                            Some(1)
                        } else {
                            None
                        },
                    }
                } else {
                    SqlType {
                        name: name.to_string(),
                        ..Default::default()
                    }
                }
            }
            Err(err) => {
                tracing::warn!(
                    "解析数据库字段[{}]数据类型,没有匹配到类型: {}",
                    self.column_name,
                    err
                );
                self.data_type.clone().into()
            }
        };
        let autoincr = is_autoincr(&self.column_default.clone().unwrap_or("".to_string()));
        Column {
            name: "".to_string(),
            column: Some(self.column_name.clone()),
            col_type: None,
            typ: sql_type,
            ignore: false,
            pk: self.primarykey,
            autoincr,
            comment: None,
            nullable: self.is_nullable.to_lowercase() == "yes",
            default: if autoincr {
                None
            } else {
                self.column_default.clone()
            },
            ..Default::default()
        }
    }
}

pub async fn get_columns<C, DB: Database>(
    context: &Context,
    conn: &mut C,
    table: &String,
    schema: Option<String>,
) -> io::Result<Vec<Column>>
where
    for<'e> &'e mut C: Executor<'e, Database = DB>,
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
    for<'a> String: Decode<'a, DB> + Type<DB>,
    for<'a> &'a str: ColumnIndex<DB::Row>,
    for<'a> std::string::String: Encode<'a, DB>,
    for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
    for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
{
    let sql: &str = r#"SELECT column_name, column_default, is_nullable, data_type, character_maximum_length,
            CASE WHEN p.contype = 'p' THEN true ELSE false END AS primarykey,
            CASE WHEN p.contype = 'u' THEN true ELSE false END AS uniquekey
        FROM pg_attribute f
            JOIN pg_class c ON c.oid = f.attrelid JOIN pg_type t ON t.oid = f.atttypid
            LEFT JOIN pg_attrdef d ON d.adrelid = c.oid AND d.adnum = f.attnum
            LEFT JOIN pg_namespace n ON n.oid = c.relnamespace
            LEFT JOIN pg_constraint p ON p.conrelid = c.oid AND f.attnum = ANY (p.conkey)
            LEFT JOIN pg_class AS g ON p.confrelid = g.oid
            LEFT JOIN INFORMATION_SCHEMA.COLUMNS s ON s.column_name=f.attname AND c.relname=s.table_name
        WHERE n.nspname= s.table_schema AND c.relkind = 'r'::char AND c.relname = $1 {} AND f.attnum > 0 ORDER BY f.attnum;"#;
    // s.replace("{}", "");

    let schema_name = if let Some(s) = schema {
        s
    } else {
        context.get_default_schema()
    };

    let query: Result<Vec<Col>, sqlx::Error> = if schema_name.is_empty() {
        let c = sql;
        sqlx::query_as::<DB, Col>(c.replace("{}", "").as_str())
            .bind(table)
            .fetch_all(&mut *conn)
            .await
    } else {
        sqlx::query_as::<DB, Col>(sql.replace("{}", "AND s.table_schema = $2").as_str())
            .bind(table)
            .bind(schema_name)
            .fetch_all(&mut *conn)
            .await
    };
    // println!("11111: {}", query);
    let mut cols: Vec<Column> = vec![];
    for col in query.unwrap().iter() {
        cols.push(col.to_column());
    }

    Ok(cols)
}

// pub async fn get_columns<'c, C, DB: Database>(
//     context: &Context,
//     conn: &mut C,
//     table: &'c String,
//     schema: Option<String>,
// ) -> io::Result<Vec<Column>>
// where
//     // // for<'a> E: 'a + sqlx::Executor<'c>,
//     // for<'a> <<E as Executor<'c>>::Database as HasArguments<'a>>::Arguments:
//     //     IntoArguments<'a, <E as Executor<'c>>::Database>,
//     for<'a> String: Decode<'a, <C as Executor<'c>>::Database> + Type<<C as Executor<'c>>::Database>,
//     for<'a> &'a str: ColumnIndex<<<C as Executor<'c>>::Database as Database>::Row>,
//     for<'a> std::string::String: Encode<'a, <C as Executor<'a>>::Database>,
//     for<'a> bool:
//         sqlx::Decode<'a, <C as Executor<'c>>::Database> + sqlx::Type<<C as Executor<'c>>::Database>,
//     for<'a> i32:
//         sqlx::Decode<'a, <C as Executor<'c>>::Database> + sqlx::Type<<C as Executor<'c>>::Database>,
//     for<'a> &'e mut C: Executor<'a, Database = DB>,
// {
// }
