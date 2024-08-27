# easy-sqlx

#### 介绍

根据结构体定义生成表结构

#### 安装教程

在 Cargo.toml 中添加引用
[dependencies]
easy-sqlx-core = { git = "https://gitee.com/knowgo/easy-sqlx.git", features = ["postgres"] }
easy-sqlx = { git = "https://gitee.com/knowgo/easy-sqlx.git" }

#### 使用说明

使用示例
定义表结构 #[derive(Table)]
#[table(
indexes [
(name = "123", columns("a", "b"))
]
)] #[index(columns("ooi"))]
struct Table1 {
// #[col(column = "key", ignore, col_type = "abc", )] #[col(column = "key", comment = "123")] #[col(pk, autoincr, len = 100)]
pub id: String, #[col(comment = "姓名", len = 20)]
pub name: Option<String>, #[col(ignore)]
pub t_o: chrono::NaiveTime,
}
// 同步表结构
// 参数 connection 为数据库连接

sync_tables(connection, vec![Table1::table()], None).await?;
