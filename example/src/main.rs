// use easy_sqlx::Table;

use easy_sqlx::{sync_tables, Table};
use easy_sqlx_core::sql::{
    builder::builder::Builder as _,
    dialects::condition::{Condition, Operator, Where},
};
// use easy_sqlx_core::sql::builder::insert_builder::InsertBuilder;
use sqlx::{postgres::PgConnectOptions, Connection, PgConnection};

#[derive(Table, Default)]
#[index(columns("name"))]
pub struct User {
    #[col(pk)]
    pub id: i64,
    #[col(len = 30)]
    pub name: String,
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
    let binding = PgConnectOptions::new()
        .host("127.0.0.1")
        .database("prjmgr")
        .username("prjmgr")
        .password("123456");
    let mut conn = PgConnection::connect_with(&binding).await.unwrap(); // .expect_err("connect database error");

    sync_tables(&mut conn, vec![User::table()], "")
        .await
        .unwrap();

    // let user = User {
    //     id: 7,
    //     name: "222".to_string(),
    //     ..Default::default()
    // };
    // // 增加完整记录
    // let a = user.insert().execute(&mut conn).await.unwrap();
    // println!("{:?}", a);
    // User::build_insert()
    //     .set_column(User::id(100))
    //     .set_column(User::name("Ok".to_string()))
    //     .execute(&mut conn)
    //     .await
    //     .unwrap();

    // let cond = Condition::And(
    //     Box::new(Condition::Condition(User::id(100), Operator::In)),
    //     Box::new(Condition::Condition(User::id(200), Operator::Le)));

    let w = Where::new().and_in(User::id_array(vec![100, 200])).and_le(User::id(200));
    // let cond = Condition::Condition(User::id(100), Operator::Ge);
    let (sql, _) = w.sql(1);
    println!("{sql}");
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
