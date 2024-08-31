// use easy_sqlx::Table;

use easy_sqlx::Table;
use easy_sqlx_core::sql::builder::pair::Pair;
use sqlx::{postgres::PgArguments, Postgres};

#[derive(Table)]
#[index(columns("name"))]
pub struct User {
    #[col(pk)]
    pub id: i64,
    #[col(len = 30)]
    pub name: String,
    pub create_time: chrono::NaiveDateTime,
}

impl User {
    pub fn id(&self) -> i64 {
        self.id
    }
}

fn main() {
    User::table();
    println!("table name: {}", User::all_cols().join(","));
    User::col_name_name();

    let pair = Pair{
        name: User::col_id_name(),
        value: 1i64,
        ..Default::default()
    };
    // pair.value.
    
    let ab: sqlx::query::Query<'_, Postgres, PgArguments> = sqlx::query::<Postgres>("");
    // let ay : PgArguments = 0;
    let e = ab.bind(pair.value);
    // e.execute()

    // sync_tables(conn, tables, default_schema)
    println!("Hello, world!");
}
