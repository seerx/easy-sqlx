// use easy_sqlx::Table;

use chrono::Local;
use easy_sqlx::{sync_tables, Table};
use easy_sqlx_core::sql::{
    builder::builder::{ExecuteBuilder, QueryBuilder},
    dialects::condition::{Where, WhereAppend},
};
// use easy_sqlx_core::sql::builder::insert_builder::InsertBuilder;
use sqlx::{postgres::PgConnectOptions, Connection, FromRow, PgConnection};
use tracing::Level;
use tracing_subscriber::{
    filter::filter_fn, layer::SubscriberExt, util::SubscriberInitExt, Layer as _, Registry,
};

#[derive(Table, Default, Debug, FromRow, Clone)]
#[index(columns("abc"))]
#[table(recreate = "now")]
pub struct User {
    #[col(pk)]
    pub id: i64,
    #[col(pk, len = 30, column = "abc")]
    pub name: String,
    pub blob: Vec<u8>,
    pub create_time: Option<chrono::NaiveDateTime>,
}

impl User {
    // pub fn id(&self) -> i64 {
    //     self.id
    // }

    // pub async fn insert(&self, conn: &mut PgConnection) -> sqlx::Result<PgQueryResult> {
    //     let table = Self::table();
    //     let schema = schema::new::<PgConnection, Postgres>("".to_owned());
    //     let sql = schema.sql_insert(&table);
    //     sqlx::query::<Postgres>(&sql)
    //         .bind(self.id)
    //         .bind(self.name.clone())
    //         .bind(self.create_time)
    //         .execute(conn)
    //         .await.map_err(|err| {
    //             println!("execute {sql} {err}");
    //             err
    //         })
    // }
}

#[tokio::main]
async fn main() {
    let layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_filter(filter_fn(|meta| {
            // println!("{}", meta.target());
            let level = *meta.level();
            level <= Level::INFO
        }));
    Registry::default().with(layer).init();

    let binding = PgConnectOptions::new()
        .host("127.0.0.1")
        .database("prjmgr")
        .username("prjmgr")
        .password("123456");
    let mut conn = PgConnection::connect_with(&binding).await.unwrap(); // .expect_err("connect database error");

    sync_tables(&mut conn, vec![User::table()], "")
        .await
        .unwrap();

    User::build_delete().execute(&mut conn).await.unwrap();

    let user = User {
        id: 106,
        name: "777".to_string(),
        create_time: Some(Local::now().naive_local()),
        ..Default::default() // ..Default::default()
    };
    // 增加完整记录
    user.insert().execute(&mut conn).await.unwrap();
    // println!("{:?}", a);

    User::build_insert()
        .set_column(User::id(14))
        .set_column(User::name("abc".to_string()))
        .set_column(User::blob(vec![]))
        .execute(&mut conn)
        .await
        .unwrap();

    let a = user.update();
    a.execute(&mut conn).await.unwrap();

    // let a = User::build_select()
    //     .columns(vec![
    //         User::col_id().to_string(),
    //         User::col_name().to_string(),
    //     ])
    //     .and(User::id_eq(11))
    //     .query().fetch(executor)
    //     .execute(&mut conn)
    //     .await.unwrap();

    User::build_update()
        .set_column(User::name("007".to_string()))
        .and(User::id_eq(7))
        .and(User::name_eq("007".to_string()))
        .execute(&mut conn)
        .await
        .unwrap();

    // User::select_by_id(100, "".to_string());
    let res: Option<User> = User::select()
        .and(User::name_like("%7%".to_string()))
        .fetch_optional(&mut conn)
        .await
        .unwrap();

    println!("{:?}", res);

    User::name_desc();

    user.delete().execute(&mut conn).await.unwrap();

    // let update = User::build_update();
    // update.and(User::id_eq(100));

    // User::create_time(val);
    // User::cre

    // User::

    // User::tes

    // user.insert(&mut conn).await.unwrap();
    // User::insert().await;

    // User::table();
    // println!("table name: {}", User::all_cols().join(","));
    // User::col_name_name();

    // let pair = Pair {
    //     name: User::col_id_name(),
    //     value: Value::Long(1),
    // };
    // pair.value.

    // let ab: sqlx::query::Query<'_, Postgres, PgArguments> = sqlx::query::<Postgres>("");
    // let ay : PgArguments = 0;
    // let e = ab.bind(1);
    // e.execute()

    // sync_tables(conn, tables, default_schema)
    println!("Hello, world!");
}
