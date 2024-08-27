pub const TABLE_NAME: &str = "__rpa_upgrade";
pub const TABLE_DDL: &str = r#"
 CREATE TABLE public.__rpa_upgrade
    (
        id BIGSERIAL,
        table_name character varying(255),
        up_tag character varying(20),
        up_value character varying(255),
        create_time character varying(20),
        PRIMARY KEY (id)
    );
"#;

pub const TABLE_INSERT: &str = r#"
 insert into public.__rpa_upgrade
    (
        table_name,
        up_tag,
        up_value,
        create_time
    )
    values 
    (
        $1, $2, $3, $4, $5
    )
"#;

pub const TABLE_QUERY: &str = r#"
    select id,
        table_name,
        up_tag,
        up_value,
        create_time 
    from public.__rpa_upgrade
    where table_name = $1 and up_tag = $2
    order by id desc
"#;

#[derive(Default, sqlx::FromRow, Debug)]
pub struct Upgrade {
    pub id: i64,
    pub table_name: String,
    pub up_tag: String,
    pub up_value: String,
    pub create_time: String,
}

// impl<'r, DB: sqlx::Database> sqlx::FromRow<'r, <DB as sqlx::Database>::Row> for Upgrade {
//     fn from_row(row: &'r <DB as sqlx::Database>::Row) -> Result<Self, sqlx::Error> {
//         todo!()
//     }
// }
