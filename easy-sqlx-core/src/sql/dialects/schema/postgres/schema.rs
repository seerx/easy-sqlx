use std::io;

use sqlx::{ColumnIndex, Database, Decode, Encode, Executor, IntoArguments};
// use tools::snowflake;
// use tracing_subscriber::fmt::format;

use crate::sql::{
    dialects::{
        condition::Where,
        context,
        page::{Order, PageRequest},
        schema::schema::Schema,
    },
    schema::{column::Column, index::Index, table::TableSchema, types::convert_sql_type},
};

use super::{
    column::get_columns,
    index::get_indexes,
    table::{get_tables, is_table_exists, sql_create_table},
    upgrade,
};

pub struct PgSchema {
    pub ctx: context::Context,
    // pub conn: PgConnection,
}

impl PgSchema {
    pub fn new(ctx: context::Context) -> impl Schema
// where
    //     for<'e> &'e mut C: Executor<'e, Database = DB>,
    //     for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
    //     for<'a> String: Decode<'a, DB> + Type<DB>,
    //     for<'a> &'a str: ColumnIndex<DB::Row>,
    //     for<'a> std::string::String: Encode<'a, DB>,
    //     for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
    //     for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
    //     for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB> + Encode<'a, DB>,
    {
        Self { ctx }
    }

    async fn check_upgrade_table<'c, E, DB: Database>(&self, conn: &mut E) -> io::Result<()>
    where
        for<'e> &'e mut E: Executor<'e, Database = DB>,
        for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
        for<'a> &'a str: ColumnIndex<DB::Row>,
        for<'a> std::string::String: Encode<'a, DB> + sqlx::Type<DB>,
        for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        // <DB as sqlx::database::HasArguments<'a>>::Arguments: sqlx::IntoArguments<'a, DB>,
    {
        let exists = is_table_exists(
            &self.ctx,
            &mut *conn,
            upgrade::TABLE_NAME.to_string(),
            "public".to_string(),
        )
        .await?;
        tracing::info!("is_table_exists: {exists}");
        if !exists {
            // 表不存在，创建
            let _ = sqlx::query(upgrade::TABLE_DDL)
                .execute(&mut *conn)
                .await
                .map_err(|err| {
                    tracing::error!(
                        "create table {} error: {} \n{}",
                        upgrade::TABLE_NAME,
                        err,
                        upgrade::TABLE_DDL
                    );
                    io::Error::new(io::ErrorKind::Other, "create upgrade table error")
                });
        }
        Ok(())
    }
}

impl Schema for PgSchema
// where
//     for<'a> &'a mut C: Executor<'a, Database = Self::DB>,
//     for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
//     for<'a> &'a str: ColumnIndex<DB::Row>,
//     for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
//     for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
//     for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB> + Encode<'a, DB>,
//     for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>,
{
    // type DB = Postgres;

    fn sql_create_table(&self, table: &TableSchema) -> std::io::Result<Vec<String>> {
        // #[cfg(feature = "postgres")]
        sql_create_table(&self.ctx, table)
    }
    fn sql_drop_table(&self, table: &TableSchema) -> String {
        self.ctx.sql_drop_table(&table.name_with_schema())
    }

    fn sql_drop_index(&self, table: &TableSchema, index: &Index) -> String {
        self.ctx
            .sql_drop_index(&table.index_name_with_schema(&index.name))
    }

    fn sql_create_index(&self, table: &TableSchema, index: &Index) -> Option<String> {
        self.ctx.sql_create_index(&table.name_with_schema(), index)
    }

    fn sql_create_indexes(&self, table: &TableSchema) -> Vec<String> {
        let mut sqls = vec![];
        if let Some(indexes) = &table.indexes {
            for idx in indexes {
                if let Some(sql) = self.sql_create_index(table, idx) {
                    sqls.push(sql)
                }
            }
        }
        sqls
    }

    async fn get_tables<C, DB: Database>(&self, conn: &mut C) -> std::io::Result<Vec<TableSchema>>
    where
        for<'a> &'a mut C: Executor<'a, Database = DB>,
        for<'a> <DB as Database>::Arguments<'a>: IntoArguments<'a, DB>,
        for<'a> &'a str: ColumnIndex<DB::Row>,
        for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB> + Encode<'a, DB>,
        for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>,
    {
        // get_tables(&self.ctx, conn).await
        // let c = &mut self.ctx.conn;
        let mut tables: Vec<TableSchema> = get_tables(&self.ctx, &mut *conn)
            .await?
            .iter()
            .map(|(name, schema)| TableSchema {
                name: name.to_owned(),
                schema: Some(schema.to_owned()),
                ..Default::default()
            })
            .collect();

        for table in tables.iter_mut() {
            #[cfg(feature = "postgres")]
            let cols =
                get_columns(&self.ctx, &mut *conn, &table.name, table.schema.clone()).await?;
            table.columns = cols;

            #[cfg(feature = "postgres")]
            let indexes =
                get_indexes(&self.ctx, &mut *conn, &table.name, table.schema.clone()).await?;
            if !indexes.is_empty() {
                table.indexes = Some(indexes);
            }
        }

        Ok(tables)
    }

    fn sql_add_column(&self, table: &TableSchema, col: &Column) -> String {
        self.ctx
            .sql_add_column(&table.name_with_schema(), &col, None, convert_sql_type)
    }

    fn is_table_name_equal(&self, table1: &TableSchema, table2: &TableSchema) -> bool {
        self.ctx
            .is_table_name_equal(&table1.name_with_schema(), &table2.name_with_schema())
    }

    fn sql_alter_column(
        &self,
        table: &TableSchema,
        old: &Column,
        new: &Column,
    ) -> io::Result<Vec<String>> {
        let table_name = &table.name_with_schema();
        self.ctx.sql_alter_column(
            table_name,
            old,
            new,
            convert_sql_type,
            new.autoincr, // 自增类型忽略默认值
        )
    }

    fn sql_drop_column(&self, table: &TableSchema, col: &Column) -> String {
        self.ctx
            .sql_drop_column(&table.name_with_schema(), &col.get_column_name())
    }

    fn table_name_with_schema(&self, table: &TableSchema) -> String {
        self.ctx.table_name_with_schema(&table.name_with_schema())
    }

    async fn query_upgrade_tags<C, DB: Database>(
        &self,
        conn: &mut C,
        table_name: &String,
        tag: &String,
    ) -> io::Result<Vec<String>>
    where
        for<'a> &'a mut C: Executor<'a, Database = DB>,
        for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
        for<'a> &'a str: ColumnIndex<DB::Row>,
        for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB> + Encode<'a, DB>,
        for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>,
    {
        // tracing::info!("query_upgrade_tags 1");
        self.check_upgrade_table(&mut *conn).await?;
        let query: Result<Vec<upgrade::Upgrade>, sqlx::Error> =
            sqlx::query_as::<DB, upgrade::Upgrade>(upgrade::TABLE_QUERY)
                .bind(table_name)
                .bind(tag)
                .fetch_all(&mut *conn)
                .await;
        // tracing::info!("query_upgrade_tags 2");
        query
            .map(|recs| {
                let values: Vec<String> = recs.iter().map(|r| r.up_value.to_owned()).collect();
                // tracing::info!("query_upgrade_tags 3 {}", values.join(","));
                values
            })
            .map_err(|err| {
                tracing::error!("query upgrade error: {:?}", err);
                io::Error::new(io::ErrorKind::Other, "check table exists error")
            })

        // Ok(vec![])
    }

    // async fn insert_upgrade_tag(
    //     &self,
    //     conn: &mut C,
    //     table_name: &String,
    //     tag: &String,
    //     tag_value: &String,
    // ) -> io::Result<()> {
    //     todo!()
    // }

    // async fn query_upgrade_tags(
    //     &self,
    //     conn: &mut C,
    //     table_name: &String,
    //     tag: &String,
    // ) -> io::Result<Vec<String>> {
    //     self.check_upgrade_table(&mut *conn).await?;
    //     let query: Result<Vec<upgrade::Upgrade>, sqlx::Error> =
    //         sqlx::query_as(upgrade::TABLE_QUERY)
    //             .bind(table_name)
    //             .bind(tag)
    //             .fetch_all(&mut *conn)
    //             .await;
    //     query
    //         .map(|recs| {
    //             let values: Vec<String> = recs.iter().map(|r| r.up_value.to_owned()).collect();
    //             values
    //         })
    //         .map_err(|err| {
    //             tracing::error!("query upgrade error: {:?}", err);
    //             io::Error::new(io::ErrorKind::Other, "check table exists error")
    //         })
    // }

    async fn insert_upgrade_tag<C, DB: Database>(
        &self,
        conn: &mut C,
        table_name: &String,
        tag: &String,
        tag_value: &String,
    ) -> io::Result<()>
    where
        for<'a> &'a mut C: Executor<'a, Database = DB>,
        for<'a> <DB as Database>::Arguments<'a>: IntoArguments<'a, DB>,
        for<'a> &'a str: ColumnIndex<DB::Row>,
        for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB> + Encode<'a, DB>,
        for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>,
    {
        self.check_upgrade_table(&mut *conn).await?;
        // tracing::error!("insert tag: {}", upgrade::TABLE_INSERT);
        sqlx::query(upgrade::TABLE_INSERT)
            // .bind(snowflake::next())
            .bind(table_name)
            .bind(tag)
            .bind(tag_value)
            .bind(chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string())
            .execute(&mut *conn)
            .await
            .map(|_| ())
            .map_err(|err| {
                tracing::error!("insert upgrade record error: {err}");
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("insert upgrade record error: {err}"),
                )
            })
        // todo!()
    }

    async fn execute_sql<'s, C, DB: Database>(
        &self,
        conn: &mut C,
        sql: &'s str,
    ) -> io::Result<<DB as Database>::QueryResult>
    where
        for<'a> &'a mut C: Executor<'a, Database = DB>,
        for<'a> DB::Arguments<'a>: IntoArguments<'a, DB>,
        for<'a> &'a str: ColumnIndex<DB::Row>,
        for<'a> bool: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i32: sqlx::Decode<'a, DB> + sqlx::Type<DB>,
        for<'a> i64: sqlx::Decode<'a, DB> + sqlx::Type<DB> + Encode<'a, DB>,
        for<'a> std::string::String: Decode<'a, DB> + Encode<'a, DB> + sqlx::Type<DB>,
    {
        sqlx::query(sql).execute(&mut *conn).await.map_err(|err| {
            tracing::error!("Execute sql {sql} error: {err}");
            io::Error::new(
                io::ErrorKind::Other,
                format!("Execute sql {sql} error: {err}"),
            )
        })
    }

    fn sql_insert(&self, table: &TableSchema) -> String {
        // let table_name = table.name_with_schema();
        // let mut column_names = "".to_string();
        // let mut column_value_holder = "".to_string();

        // for (n, col) in table.columns.iter().enumerate() {
        //     if n > 0 {
        //         column_names.push_str(",");
        //         column_value_holder.push_str(",");
        //     }
        //     column_names.push_str(col.get_column_name().as_str());
        //     column_value_holder.push_str(format!("${}", n + 1).as_str());
        // }

        // format!(
        //     "insert into {} ({column_names}) values ({column_value_holder})",
        //     self.ctx.quote(&self.table_name_with_schema(table))
        // )

        self.sql_insert_columns(
            table,
            &table
                .columns
                .iter()
                .map(|c| c.get_column_name())
                .collect::<Vec<String>>(),
        )
    }

    fn sql_insert_columns(&self, table: &TableSchema, cols: &Vec<String>) -> String {
        // let table_name = table.name_with_schema();
        let mut column_names = "".to_string();
        let mut column_value_holder = "".to_string();

        for (n, col) in cols.iter().enumerate() {
            if n > 0 {
                column_names.push_str(",");
                column_value_holder.push_str(",");
            }
            column_names.push_str(self.ctx.quote(&col).as_str());
            column_value_holder.push_str(format!("${}", n + 1).as_str());
        }

        format!(
            "insert into {} ({column_names}) values ({column_value_holder})",
            self.ctx.quote(&self.table_name_with_schema(table))
        )
    }

    fn sql_update_columns(
        &self,
        table: &TableSchema,
        cols: &Vec<String>,
        wh: Option<Where>,
    ) -> String {
        let mut columns = "".to_string();

        for (n, col) in cols.iter().enumerate() {
            if n > 0 {
                columns.push_str(",");
            }
            columns.push_str(self.ctx.quote(&col).as_str());
            columns.push_str("=");
            columns.push_str(format!("${}", n + 1).as_str());
        }

        let mut where_str = String::from("");
        if let Some(w) = wh {
            let (ws, _) = w.sql(cols.len() + 1, &self.quoter());
            if !ws.is_empty() {
                where_str.push_str(" where ");
                where_str.push_str(&ws);
            }
        }

        format!(
            "update {} set {columns} {where_str}",
            self.ctx.quote(&self.table_name_with_schema(table))
        )
    }

    fn quoter(&self) -> crate::sql::utils::quote::Quoter {
        self.ctx.quoter.clone()
    }

    fn sql_delete(&self, table: &TableSchema, wh: Option<Where>) -> String {
        let mut where_str = String::from("");
        if let Some(w) = wh {
            let (ws, _) = w.sql(1, &self.quoter());
            if !ws.is_empty() {
                where_str.push_str(" where ");
                where_str.push_str(&ws);
            }
        }

        format!(
            "delete from {} {where_str}",
            self.ctx.quote(&self.table_name_with_schema(table))
        )
    }

    fn sql_count(&self, table: &TableSchema, wh: Option<Where>) -> String {
        let mut where_str = String::from("");
        if let Some(w) = wh {
            let (ws, _) = w.sql(1, &self.quoter());
            if !ws.is_empty() {
                where_str.push_str(" where ");
                where_str.push_str(&ws);
            }
        }

        format!(
            r#"select count(0) as "count" from {} {where_str}"#,
            self.ctx.quote(&self.table_name_with_schema(table))
        )
    }

    fn sql_select(
        &self,
        table: &TableSchema,
        wh: Option<Where>,
        orders: &Vec<Order>,
        pg: Option<&PageRequest>,
    ) -> String {
        let cols: Vec<String> = table
            .columns
            .iter()
            .map(|c| c.get_query_column_name())
            .collect();
        self.sql_select_columns(table, &cols, wh, orders, pg)
        // let cols: Vec<String> = table
        //     .columns
        //     .iter()
        //     .map(|c| self.quoter().quote(&c.get_query_column_name()))
        //     .collect();

        // let mut where_str = String::from("");
        // if let Some(w) = wh {
        //     let (ws, _) = w.sql(1, &self.quoter());
        //     if !ws.is_empty() {
        //         where_str.push_str(" where ");
        //         where_str.push_str(&ws);
        //     }
        // }

        // let mut order_str = String::from("");
        // if !orders.is_empty() {
        //     order_str.push_str(" order by ");
        //     let items: Vec<String> = orders
        //         .iter()
        //         .map(|o| format!("{} {}", self.ctx.quote(&o.field), o.order_type.sql()))
        //         .collect();
        //     order_str.push_str(items.join(", ").as_str());
        // }

        // let mut page_str = String::from("");
        // if let Some(page) = pg {
        //     // 分页
        //     page_str.push_str(
        //         format!(
        //             "offset {} limit {}",
        //             (page.page_no - 1) * page.page_size,
        //             page.page_size
        //         )
        //         .as_str(),
        //     );
        // }

        // format!(
        //     "select {} from {} {where_str} {order_str} {page_str}",
        //     cols.join(","),
        //     self.ctx.quote(&self.table_name_with_schema(table))
        // )
    }

    fn sql_select_columns(
        &self,
        table: &TableSchema,
        columns: &Vec<String>,
        wh: Option<Where>,
        orders: &Vec<Order>,
        pg: Option<&PageRequest>,
    ) -> String {
        let cols: Vec<String> = columns.iter().map(|c| self.quoter().quote(&c)).collect();

        let mut where_str = String::from("");
        if let Some(w) = wh {
            let (ws, _) = w.sql(1, &self.quoter());
            if !ws.is_empty() {
                where_str.push_str(" where ");
                where_str.push_str(&ws);
            }
        }

        let mut order_str = String::from("");
        if !orders.is_empty() {
            order_str.push_str(" order by ");
            let items: Vec<String> = orders
                .iter()
                .map(|o| format!("{} {}", self.ctx.quote(&o.field), o.order_type.sql()))
                .collect();
            order_str.push_str(items.join(", ").as_str());
        }

        let mut page_str = String::from("");
        if let Some(page) = pg {
            // 分页
            page_str.push_str(
                format!(
                    "offset {} limit {}",
                    (page.get_page_no() - 1) * page.get_page_size(),
                    page.get_page_size()
                )
                .as_str(),
            );
        }

        format!(
            "select {} from {} {where_str} {order_str} {page_str}",
            cols.join(","),
            self.ctx.quote(&self.table_name_with_schema(table))
        )
    }

    // async fn execute_sql<'c, E>(
    //     &self,
    //     conn: E,
    //     sql: &'c str,
    // ) -> sqlx::Result<<<E as sqlx::Executor<'c>>::Database as sqlx::Database>::QueryResult>
    // where
    //     for<'a> E: sqlx::Executor<'a> + sqlx::Executor<'a>,
    //     for<'a> <<E as Executor<'a>>::Database as HasArguments<'a>>::Arguments:
    //         IntoArguments<'a, <E as Executor<'a>>::Database>,
    // {
    //     sqlx::query(sql).execute(conn).await
    // }

    // async fn execute_sql<'a>(
    //     &self,
    //     conn: E,
    //     sql: &'a str,
    // ) -> sqlx::Result<<<E as sqlx::Executor<'a>>::Database as sqlx::Database>::QueryResult> {
    //     sqlx::query(sql).execute(conn).await
    //     // .map(|_| Ok(()))
    //     // .map_err(|err| {
    //     //     io::Error::new(io::ErrorKind::Other, format!("Execute sql: {sql}\n{err}"))
    //     // })
    //     // Ok(())
    // }

    // async fn execute1<'a>(
    //     sql: &'a str,
    //     executor: E,
    // ) -> sqlx::Result<<<E as sqlx::Executor<'a>>::Database as sqlx::Database>::QueryResult> {
    //     sqlx::query(sql).execute(executor).await
    // }

    // type Database = Postgres;
}

// impl Schema for PgSchema<'a> {
//     type Connection = PgConnection;
//     fn get_context(&self) -> &'a context::Context {
//         self.ctx
//     }

//     fn get_connection(&mut self) -> &'a mut PgConnection {
//         self.conn
//     }
// }
