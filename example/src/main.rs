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

impl User {
    pub fn id(&self) -> i64 {
        self.id
    }
}

fn main() {
    User::table();
    println!("table name: {}", User::all_cols().join(","));
    User::col_name_name();
    // sync_tables(conn, tables, default_schema)
    println!("Hello, world!");
}
