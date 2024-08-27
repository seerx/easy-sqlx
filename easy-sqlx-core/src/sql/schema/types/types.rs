// use proc_macro2::{TokenStream};
use quote::{quote, ToTokens};
use regex::Regex;

use super::find_relation;

pub struct TypeRelation {
    pub rust: &'static str,
    pub sql: &'static str,
    pub maybe_types: Option<Vec<&'static str>>,
    /// sql 类型的固有长度
    pub fix_len: Option<isize>,
    /// 没有指定长度时，使用此长度值
    pub default_len: Option<isize>,
    // pub a: chrono::NaiveDate,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct SqlType {
    pub name: String,
    pub len: Option<isize>,
    pub len2: Option<isize>,
    pub fixed_len: Option<isize>,
}

// impl PartialEq for SqlType {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name && self.len == other.len && self.len2 == other.len2 && self.fixed_len == other.fixed_len
//     }
// }

// impl Eq for SqlType {

// }

impl SqlType {
    pub fn new(rust_type: &String, len: Option<isize>) -> Self {
        match find_relation(rust_type) {
            Ok(rel) => Self {
                name: rel.sql.to_string(),
                len: if rel.fix_len.is_some() {
                    // 固定长度的 sql 数据类型
                    Some(rel.fix_len.unwrap())
                } else if len.is_some() {
                    // 提供了长度
                    len
                } else {
                    // 未提供长度
                    if rel.default_len.is_some() {
                        // 有默认长度，则使用默认长度
                        rel.default_len
                    } else {
                        // 没有默认长度，则返回 None
                        None
                    }
                },
                // else if rust_type == "String" {
                //     // 字符串类型一定要有长度，如果未设置，则为 255
                //     if len.is_none() {
                //         Some(255)
                //     } else {
                //         len
                //     }
                // } else {
                //     // 其它类型使用设置的 len
                //     len
                // },
                fixed_len: rel.fix_len.clone(),
                ..Default::default()
            },
            Err(err) => panic!("{}", err),
        }
    }
}

impl From<&String> for SqlType {
    fn from(value: &String) -> Self {
        let reg = Regex::new(r#"(\w+)\((\s*\d+\s*[,\s*\d+\s*]*)\)"#).unwrap();
        let mut len: Option<isize> = None;
        let mut len2: Option<isize> = None;
        let mut type_name = "".to_string();
        if reg.is_match(&value) {
            for (_, [name, len_str]) in reg.captures_iter(&value).map(|c| c.extract()) {
                // println!("{} - {}", name, len_str);
                let lens: Vec<&str> = len_str.split(",").into_iter().map(|s| s.trim()).collect();
                if lens.len() > 0 {
                    // 多于 1 个长度参数
                    len = Some(lens[0].parse::<isize>().unwrap());
                }
                if lens.len() > 1 {
                    // 多于 1 个长度参数
                    len2 = Some(lens[1].parse::<isize>().unwrap());
                }
                type_name = name.to_string();
                break;
            }
        } else {
            type_name = value.clone();
        }
        Self {
            name: type_name.to_uppercase(),
            len,
            len2,
            fixed_len: None,
        }
    }
}

impl From<String> for SqlType {
    fn from(value: String) -> Self {
        (&value).into()
    }
}

impl ToTokens for SqlType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // 将 MyStruct 转换为 TokenStream，这里以简单的 "struct_name" 为例
        let name = self.name.clone();
        let has_len = self.len.is_some();
        let len = self.len.unwrap_or(0);

        let has_len2 = self.len2.is_some();
        let len2 = self.len2.unwrap_or(0);
        let has_fixed = self.fixed_len.is_some();
        let fix_len = self.fixed_len.unwrap_or(0);
        // let len = self.len.clone();
        quote! {
            easy_sqlx_core::sql::schema::types::types::SqlType {
                name: #name.to_string(),
                len: if #has_len { Some(#len) } else { None },
                len2: if #has_len2 { Some(#len2) } else { None },
                fixed_len: if #has_fixed { Some(#fix_len) } else { None },
            }
        }
        .to_tokens(tokens);

        // let struct_name = TokenTree::Ident(syn::Ident::new("struct_name", proc_macro2::Span::call_site()));
        // let token_stream = TokenStream::from_iter(vec![struct_name]);
        // tokens.extend(token_stream);
        // TokensOrDefault(&self.name).to_tokens(tokens);
        // TokenStream::from("name".to_string()).append(tokens);
        // self.vis.to_tokens(tokens);
        // match &self.data {
        //     Data::Struct(d) => d.struct_token.to_tokens(tokens),
        //     Data::Enum(d) => d.enum_token.to_tokens(tokens),
        //     Data::Union(d) => d.union_token.to_tokens(tokens),
        // }
        // self.ident.to_tokens(tokens);
        // self.generics.to_tokens(tokens);
        // match &self.data {
        //     Data::Struct(data) => match &data.fields {
        //         Fields::Named(fields) => {
        //             self.generics.where_clause.to_tokens(tokens);
        //             fields.to_tokens(tokens);
        //         }
        //         Fields::Unnamed(fields) => {
        //             fields.to_tokens(tokens);
        //             self.generics.where_clause.to_tokens(tokens);
        //             TokensOrDefault(&data.semi_token).to_tokens(tokens);
        //         }
        //         Fields::Unit => {
        //             self.generics.where_clause.to_tokens(tokens);
        //             TokensOrDefault(&data.semi_token).to_tokens(tokens);
        //         }
        //     },
        //     Data::Enum(data) => {
        //         self.generics.where_clause.to_tokens(tokens);
        //         data.brace_token.surround(tokens, |tokens| {
        //             data.variants.to_tokens(tokens);
        //         });
        //     }
        //     Data::Union(data) => {
        //         self.generics.where_clause.to_tokens(tokens);
        //         data.fields.to_tokens(tokens);
        //     }
        // }
    }
}

// impl ToString for SqlType {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }

impl std::fmt::Display for SqlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(len2) = self.len2 {
            if let Some(len) = self.len {
                write!(f, "({}, {})", len, len2)?;
            }
        } else {
            if let Some(len) = self.len {
                write!(f, "({})", len)?;
            }
        }
        Ok(())
    }
}

// #[macro_export]
// macro_rules! sql_type {
//     ($tp:ident) => {
//         $crate::sql::schema::types::types::SqlType {
//             name: $crate::sql::schema::types::names::INT.to_string(),
//             ..$crate::sql::schema::types::types::SqlType::default()
//         }
//     };
//     ($tp:ident, $len:tt) => {
//         $crate::sql::schema::types::types::SqlType {
//             name: $crate::sql::schema::types::names::VARCHAR.to_string(),
//             len: $len,
//             ..$crate::sql::schema::types::types::SqlType::default()
//         }
//     };
//     (i8) => {
//         $crate::sql::schema::types::types::SqlType {
//             name: $crate::sql::schema::types::names::INT.to_string(),
//             ..$crate::sql::schema::types::types::SqlType::default()
//         }
//     };
//     (u8) => {
//         $crate::sql::schema::types::types::SqlType {
//             name: $crate::sql::schema::types::names::INT.to_string(),
//             ..$crate::sql::schema::types::types::SqlType::default()
//         }
//     };
//     (i16) => {
//         $crate::sql::schema::types::types::SqlType {
//             name: $crate::sql::schema::types::names::INT.to_string(),
//             ..$crate::sql::schema::types::types::SqlType::default()
//         }
//     };
//     (u16) => {
//         $crate::sql::schema::types::types::SqlType {
//             name: $crate::sql::schema::types::names::INT.to_string(),
//             ..$crate::sql::schema::types::types::SqlType::default()
//         }
//     };
//     (i32) => {
//         $crate::sql::schema::types::types::SqlType {
//             name: $crate::sql::schema::types::names::INT.to_string(),
//             ..$crate::sql::schema::types::types::SqlType::default()
//         }
//     };
//     (u32) => {
//         $crate::sql::schema::types::types::SqlType {
//             name: $crate::sql::schema::types::names::INT.to_string(),
//             ..$crate::sql::schema::types::types::SqlType::default()
//         }
//     };
//     (String, $len:literal) => {
//         $crate::sql::schema::types::types::SqlType {
//             name: $crate::sql::schema::types::names::VARCHAR.to_string(),
//             len: Some($len),
//             ..$crate::sql::schema::types::types::SqlType::default()
//         }
//     };
//     (&str, $len:literal) => {
//         $crate::sql::schema::types::types::SqlType {
//             name: $crate::sql::schema::types::names::VARCHAR.to_string(),
//             len: Some($len),
//             ..$crate::sql::schema::types::types::SqlType::default()
//         }
//     };
// }
