# easy-sqlx

#### 介绍

根据结构体定义同步生成数据库表结构，当前仅支持 postgres 数据库。

#### 要求

##### 1 sqlx 版本 >= 0.8
##### 2 尽量不要使用 query 宏，因为表是根据结构体定义动态生成，query 宏可能会造成不必要的麻烦

#### 安装教程

在 Cargo.toml 中添加引用
```
[dependencies]
easy-sqlx-core = { git = "https://gitee.com/knowgo/easy-sqlx.git", features = ["postgres"] }
easy-sqlx = { git = "https://gitee.com/knowgo/easy-sqlx.git" }
```
#### 使用说明

##### 同步表结构
定义表结构 #[derive(Table)]
```
#[derive(Table)]
#[table(
    indexes [
        (name = "123", columns("a", "b"))
    ]
)] 
#[index(columns("name"))]
struct Table1 {
    #[col(column = "key", comment = "123")]
    #[col(pk, autoincr, len = 100)]
    pub id: String,
    #[col(comment = "姓名", len = 20)]
    pub name: Option<String>,
    #[col(ignore)]
    pub t_o: chrono::NaiveTime,
}
```
同步表结构，参数 connection 为数据库连接
```
sync_tables(connection, vec![Table1::table()], None).await?;
```
table 属性
    name            表名称
    comment         注释
    schema          表空间
    from            从另一个表 rename
    recreate        重新创建表,在同一个数据库中不会重复操作，直到修改其值
    trim_columns    清理 struct 中未定义的数据库列
    trim_indexes    清理 table 未定义的索引
    indexes         索引数组，参见索引

index 属性
    name            索引名称
    columns()       索引的列，字符串数组，使用小括号包围
    unique          是否唯一索引

col 属性
    ignore          忽略，不与数据库关联
    pk              主键
    autoincr        自增类型
    column          字段名称
    len             长度，字符串长度或精度
    col_type        sql 数据类型，用于自定义数据库类型
    comment         说明
    default         默认值
    from            从另一个字段重命名而来
    replace         如果修改数据类型发生错误时，删除原字段，重新创建


##### 添加记录 1
```
let table = Table1 {
    id: "test001".to_string(),
    name: "test".to_string(), 
    ..Default::default()
};
// 增加完整记录
table.insert().execute(&mut conn).await.unwrap();
```

##### 添加记录 2
```
Table1::build_insert()
    .set_column(Table1::id("11".to_string()))
    .execute(&mut conn)
    .await
    .unwrap();
```