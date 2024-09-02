// use easy_sqlx::Table;

use easy_sqlx::{sync_tables, Table};
use easy_sqlx_core::sql::{builder::{builder::Builder, easy_insert_builder::InsertBuilder}, dialects::schema::{self, schema::Schema}, utils::{pair::Pair, value::Value}};
use sqlx::{
    postgres::{PgArguments, PgConnectOptions, PgQueryResult},
    Connection, PgConnection, Postgres,
};

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
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn insert1<'a>(&self) -> InsertBuilder<'a> {
        
        let mut builder: InsertBuilder<'a> = InsertBuilder::new(Self::table());
        for col in Self::table().columns {
            let p = Pair{
                name: col.get_column_name(),
                value: Value::from(&self.id),
            };
            builder.add_column(p);

            let p = Pair {
                name: col.get_column_name(),
                value: Value::from(&self.name),
            };
            builder.add_column(p);
        }
        
        builder
    }

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

    let user = User {
        id: 3,
        name: "222".to_string(),
        ..Default::default()
    };

    let a = user.insert();
    a.execute(&mut conn).await.unwrap();
    println!("{:?}", a);

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
