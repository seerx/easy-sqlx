# easy-sqlx

#### 介绍

根据结构体定义同步生成数据库表结构，简化增删改操作和大部分的单表查询操作，当前仅支持 postgres 数据库。

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
use easy_sqlx::WhereAppend; // 引用 WhereAppend

#[derive(Table, FromRow, Debug)]
#[table(
    indexes [
        (name = "123", columns("a", "b"))
    ]
)]
#[index(columns("name"))]
struct User {
    #[col(column = "key", comment = "123")]
    #[col(pk)]
    pub id: i64,
    #[col(comment = "姓名", len = 20)]
    pub name: Option<String>,
    #[col(ignore)]
    pub create_time: Option<chrono::NaiveTime>,
}
```

<pre>
table 属性
    name            表名称
    comment         注释
    schema          表空间
    from[未完成]    从另一个表 rename
    recreate        重新创建表,在同一个数据库中不会重复操作，直到修改其值
    trim_columns    清理 struct 中未定义的数据库列
    trim_indexes    清理 table 未定义的索引
    indexes         索引数组，参见索引

index 属性，可定义在 table 属性内，也可以单独定义到 struct
    name            索引名称
    columns()       索引的列，字符串数组，使用小括号包围
    unique          是否唯一索引

col 属性,Option 包裹的列为 nullable ，否则为必填
    ignore          忽略，不与数据库关联
    pk              主键
    autoincr        自增类型
    column          字段名称
    len             长度，字符串长度或精度
    col_type        sql 数据类型，用于自定义数据库类型
    comment         说明
    default         默认值
    from[未完成]    从另一个字段重命名而来 
    replace         如果修改数据类型发生错误时，删除原字段，重新创建
</pre>

同步表结构，参数 connection 为数据库连接

```
sync_tables(connection, vec![User::table()]).await?;
```

##### 添加记录 1

```
let user = User {
    id: 1,
    name: Some("test".to_string()),
    ..Default::default()
};
user.insert().execute(&mut conn).await.unwrap();
```

##### 添加记录 2

```
User::build_insert()
    .set(User::id(2)) // 设置字段值，未设置的为 null
    .execute(&mut conn)
    .await
    .unwrap();
```

##### 修改 1

```
let user = User {
    id: 1,
    name: Some("test---1".to_string()),
    ..Default::default()
};
user.update().execute(&mut conn).await.unwrap();
```

##### 修改 2

```
User::build_update()
        .set(User::name("007".to_string()))
        .and(User::id_eq(2))
        .execute(&mut conn)
        .await
        .unwrap();
```

##### 删除 1

```
let user = User {
    id: 1, // 主键值为 1
    name: Some("test---1".to_string()),
    ..Default::default()
};
// 根据主键 删除
user.delete().execute(&mut conn).await.unwrap();
```

##### 删除 2

```
User::build_delete()
        .and(User::id_eq(2)) // 删除 id 为 2 的记录
        .execute(&mut conn).await.unwrap();
```

##### 查询
```
let u: User = User::select_by_id(1) // 联合主键会有多个参数
.one(&mut conn).await.unwrap();
println!("{:?}", u);

/// 通用查询
User::select() // 生成 SelectBuilder
    .and(User::id_eq(1)) // 查询条件 id = 1
    .one(&mut conn).await.unwrap();
```
<pre>
SelectBuilder 提供了查询条件的添加和组合功能和排序条件的添加
and 和 or 为添加查询条件函数，首次添加条件时 and 和 or 功能一致，再次添加时
and(cond) : cond 将于前面所有的条件进行 and 操作
or(cond) : cond 将于前面所有的条件进行 or 操作
参数 cond 也可以是 Where 对象，Where 对象可以包含多个条件
实际上，查询本身的条件也是 Where 对象。

SelectBuilder 提供以下几种获取数据的方法
one                 获取一条记录
optional            获取一条记录，如果不存在返回 None
all                 获取全部记录
page                分页查询
count               查询记录数
one_scalar          获取一个标量
optional_scalar     获取一个可选标量，如果不存在返回 None
all_scalars         获取全部标量
page_scalars        分页获取标量 
</pre>