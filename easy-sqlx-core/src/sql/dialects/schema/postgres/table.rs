use std::io::{self, Error};

use sqlx::{ColumnIndex, Database, Decode, Encode, Executor, IntoArguments, Type};

use crate::sql::{
    dialects::context::Context,
    schema::{self, types::convert_sql_type},
};

// SQLCreateTable 生成创建表结构的 SQL
pub fn sql_create_table(
    context: &Context,
    table: &schema::table::TableSchema,
) -> io::Result<Vec<String>> {
    if table.columns.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("表 {} 没有定义字段", table.name),
        ));
    }

    let mut create_table = "CREATE TABLE IF NOT EXISTS ".to_string();
    let table_name = context.table_name_with_schema(&table.name_with_schema());

    create_table.push_str(context.quote(&table_name).as_str());
    create_table.push_str(" (");

    let mut pks: Vec<String> = vec![];
    // 查找全部主键
    for col in &table.columns {
        if col.pk {
            pks.push(col.get_column_name());
        }
    }

    let mut cols = vec![];
    for col in &table.columns {
        // p.SQLColumn(col, len(pkList) == 1)
        let col_def = context.sql_column(col, pks.len() == 1, None, convert_sql_type);
        cols.push(col_def);
    }
    // // inlinePK := len(pkList) == 1
    // // var cols = []string{}
    // for _, col := range table.Columns {
    // 	// col := schema.GetColumn(n)
    // 	s, err := p.SQLColumn(col, len(pkList) == 1)
    // 	if err != nil {
    // 		return nil, err
    // 	}
    // 	cols = append(cols, s)
    // }
    create_table.push_str(cols.join(",").as_str());
    // sql += strings.Join(cols, ",")
    if pks.len() > 1 {
        create_table.push_str(", PRIMARY KEY (");
        create_table.push_str(pks.join(",").as_str());
        create_table.push_str(")");
    }
    // if len(pkList) > 1 {
    // 	sql += ", PRIMARY KEY ( "
    // 	sql += quoter.Join(pkList, ",")
    // 	sql += " ) "
    // }
    // sql = strings.TrimSpace(sql)
    // }
    // sql += ")"
    create_table.push_str(")");

    Ok(vec![create_table])
}

#[derive(Default, sqlx::FromRow, Debug)]
struct Exists {
    pub exists: bool,
}

// SQLTableExists 生成判断表是否存在的 SQL
pub async fn is_table_exists<'c, C, DB: Database>(
    context: &Context,
    conn: &mut C,
    table_name: String,
    schema: String,
) -> io::Result<bool>
where
    for<'e> &'e mut C: Executor<'e, Database = DB>,
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
    // for<'a> <<E as Executor<'c>>::Database as HasArguments<'a>>::Arguments:
    //     IntoArguments<'a, <E as Executor<'c>>::Database>,
    for<'a> std::string::String: Encode<'a, DB> + sqlx::Type<DB>,
    for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
    for<'a> &'a str: ColumnIndex<DB::Row>,
{
    let schema_name = if schema.is_empty() {
        context.get_default_schema()
    } else {
        schema.clone()
    };

    let query: Result<Exists, sqlx::Error> = if schema_name.is_empty() {
        sqlx::query_as::<DB, Exists>(
            r#"SELECT EXISTS (
            SELECT FROM information_schema.tables
            WHERE  table_name = $1
        )"#,
        )
        .bind(table_name)
        .fetch_one(&mut *conn)
        .await
    } else {
        sqlx::query_as(
            r#"SELECT EXISTS (
            SELECT FROM information_schema.tables
            WHERE table_schema = $1
          AND table_name = $2
        )"#,
        )
        .bind(schema_name)
        .bind(table_name)
        .fetch_one(conn)
        .await
    };

    query
        .map_err(|err| {
            tracing::error!("check table exists error: {:?}", err);
            Error::new(io::ErrorKind::Other, "check table exists error")
        })
        .map(|exists| exists.exists)

    // Ok(false)
    // args := []interface{}{tableName}
    // return "select tablename from pg_tables where tablename=$1", args
}

#[derive(Default, sqlx::FromRow)]
pub struct Table {
    pub tablename: String,
    pub schemaname: String,
}

/// 查询表
pub async fn get_tables<'c, C, DB: Database>(
    context: &Context,
    conn: &mut C,
) -> io::Result<Vec<(String, String)>>
where
    for<'e> &'e mut C: Executor<'e, Database = DB>,
    for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
    for<'a> String: Decode<'a, DB> + Type<DB>,
    for<'a> &'a str: ColumnIndex<DB::Row>,
    for<'a> std::string::String: Encode<'a, DB>,
{
    // sqlx::query_as("").fetch_all(conn).await;
    // Table::
    // PgArguments::``
    let query: Result<Vec<Table>, sqlx::Error> = if context.get_default_schema().is_empty() {
        // PostgresExecutor::find_all(r#"select tablename, schemaname from pg_tables"#, conn).await
        sqlx::query_as(r#"select tablename, schemaname from pg_tables"#)
            .fetch_all(conn)
            .await
    } else {
        // let query: Result<Vec<Table>, sqlx::Error> =
        sqlx::query_as(r#"select tablename, schemaname from pg_tables where schemaname=$1"#)
            .bind(context.get_default_schema())
            .fetch_all(conn)
            .await
    };
    query
        .map(|tables| {
            tables
                .iter()
                .map(|t| (t.tablename.to_owned(), t.schemaname.to_owned()))
                .collect()
        })
        .map_err(|err| {
            tracing::error!("query tables error: {:?}", err);
            Error::new(io::ErrorKind::Other, "query tables error")
        })
}
