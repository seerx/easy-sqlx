// use easy_sqlx::Table;

use easy_sqlx::Table;

#[derive(Table)]
#[index(columns("name"))]
pub struct User {
    #[col(pk)]
    pub id: i64,
    #[col(len = 30)]
    pub name: String,
    pub create_time: chrono::NaiveDateTime,
}

fn main() {
    User::table();
    // sync_tables(conn, tables, default_schema)
    println!("Hello, world!");
}
