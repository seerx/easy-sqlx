use types::TypeRelation;

use super::column::Column;

#[cfg(feature = "postgres")]
mod postgres;
pub mod rust_types;
pub mod sql_types;
pub mod types;

pub fn convert_sql_type(col: &Column) -> String {
    #[cfg(feature = "postgres")]
    postgres::convert_sql_type(col)
}

// #[cfg(feature = "postgres")]
// lazy_static! {
//     static ref TR = postgres::TYPE_REPLATIONS;
// }
/// 使用 rust 类型查找 与 sql 对应关系
pub fn find_relation(rust_type: &String) -> std::io::Result<&'static TypeRelation> {
    #[cfg(feature = "postgres")]
    let tr: Option<&TypeRelation> = postgres::TYPE_REPLATIONS
        .iter()
        .find(|tr| tr.rust == rust_type.as_str());
    if tr.is_some() {
        return Ok(tr.unwrap());
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("找不到对应的类型: {}", rust_type).as_str(),
    ))
}

/// 使用 sql 类型查找 与 sql 对应关系
pub fn find_relation_by_sql_type(sql_type: &String) -> std::io::Result<&'static TypeRelation> {
    #[cfg(feature = "postgres")]
    let tr: Option<&TypeRelation> = postgres::TYPE_REPLATIONS
        .iter()
        .find(|tr| tr.sql == sql_type.as_str());
    if tr.is_some() {
        return Ok(tr.unwrap());
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("找不到对应的类型: {}", sql_type).as_str(),
    ))
}

// pub  find_relation = names::find_relation;

/// 从数据库的数据类型，匹配到生成该类型使用的 sql_type，
/// 用于读取数据库列属性
pub fn match_sql_type(sql_type: &String) -> std::io::Result<&'static str> {
    #[cfg(feature = "postgres")]
    let tr: Option<&TypeRelation> = postgres::TYPE_REPLATIONS.iter().find(|tr| {
        tr.sql == sql_type.to_uppercase().as_str()
            || if tr.maybe_types.is_some() {
                tr.maybe_types
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|item| *item == sql_type.to_uppercase().as_str())
            } else {
                false
            }
    });
    if tr.is_some() {
        return Ok(tr.unwrap().sql);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("不能匹配的类型: {}", sql_type).as_str(),
    ))
}
