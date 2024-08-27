use easy_sqlx_core::sql::schema::{column::Column, types::types::SqlType};
use easy_sqlx_utils::option_parser::parse_type_options;
use syn::{spanned::Spanned, Error, Field};

pub fn parse_column_attrs(field: &Field) -> syn::Result<Option<Column>> {
    // let attr

    let ident_item = &field.ident;
    // 解析结构体字段名称
    let field_name = ident_item.clone().unwrap().to_string();
    // 解析数据类型

    let (options, rust_type, _) = parse_type_options(&field.ty);
    let has_option: bool = options > 0;

    let mut column = Column {
        name: field_name.clone(),
        // comment: Some(format!("{} - {}", options.to_string(), has_option)),
        nullable: has_option,
        typ: SqlType::new(&rust_type, None),
        ..Column::default()
    };

    // field.attrs.iter().find(|attr| {})
    for attr in field.attrs.iter() {
        if attr.path().is_ident("col") {
            // 有 col 属性
            match attr.parse_args::<Column>() {
                Ok(col) => {
                    if col.ignore {
                        // 忽略该字段
                        return Ok(None);
                    }

                    if rust_type == "String" {
                        // 解析字符串长度
                        if col.typ.len.is_some() {
                            column.typ = SqlType::new(
                                &rust_type,
                                col.typ.len,
                            );
                        }
                    }
                    column.assign(&col);
                }
                Err(err) => {
                    return Err(Error::new(
                        err.span(),
                        format!("{err} on field {field_name}"),
                    ));
                }
            }
        }
        if attr.path().is_ident("table") {
            return Err(Error::new(
                attr.span(),
                format!("Invalid #[table()] on field {field_name}"),
            ));
        }
    }

    // if let Some(res) = &mut field.attrs.iter().find_map(|attr| {
    //     // field_name = format!("{field_name}_1");
    //     if attr.path().is_ident("col") {
    //         // field_name = format!("{field_name}_1_col");
    //         return Some(attr.parse_args::<Column>());
    //     }
    //     None
    // }) {
    //     match res {
    //         Ok(col) => {
    //             if col.ignore {
    //                 return Ok(None);
    //             }
    //             col.name = field_name; // format!("{field_name} - {}", col.column.clone().unwrap_or("---".to_string()));
    //             col.nullable = has_option;
    //             let typ = if rust_type == "String" {
    //                 // 解析字符串长度
    //                 // 解析 len 值
    //                 easy_sqlx_core::sql::schema::types::types::SqlType::new(&rust_type, col.typ.len)
    //             } else {
    //                 // 解析其它类型
    //                 easy_sqlx_core::sql::schema::types::types::SqlType::new(&rust_type, None)
    //             };
    //             col.typ = typ;
    //             Ok(Some((*col).clone()))
    //         }
    //         Err(err) => Err((*err).clone()),
    //     }
    // } else {
    //     // 没有  col 属性
    //     let typ = easy_sqlx_core::sql::schema::types::types::SqlType::new(&rust_type, None);
    //     Ok(Some(Column {
    //         name: field_name,
    //         typ,
    //         // comment: Some("NONW".to_string()),
    //         nullable: has_option,
    //         ..Default::default()
    //     }))
    // }
    Ok(Some(column))
}

#[derive(Debug, Eq, PartialEq)]
pub struct Col {
    // Anything that implements `syn::parse::Parse` is supported.
    // mandatory_type: syn::Type,
    // mandatory_ident: syn::Ident,
    pub ignore: Option<syn::Lit>,
    pub name: Option<syn::Lit>,
    pub col_type: Option<syn::Lit>,
    pub pk: Option<syn::Lit>,
    pub autoincr: Option<syn::Lit>,
    pub len: Option<syn::Lit>,
    pub comment: Option<syn::Lit>,
}

// pub fn parse_column(field: &Field) -> std::io::Result<TokenStream> {
//     let ident_item = &field.ident;
//     // 解析结构体字段名称
//     let mut field_name = ident_item.clone().unwrap().to_string();
//     // 解析数据类型

//     let (options, rust_type, _) = parse_type_options(&field.ty);
//     let has_option: bool = options > 0;

//     // let col = &field.attrs[0].parse_args::<Column>().unwrap();
//     // 测试，用于显示原始数据类型
//     // field_name = format!("{}:{}", &field_name, col.col_name.clone().unwrap_or("NONE".to_string()));
//     // field.attrs[0].to_tokens(tokens)
//     // for attr in field.attrs {
//     //     match attr.path().get_ident() {
//     //         Some(ident) if ident == "" => {
//     //             // return Some(attr..tokens.clone());
//     //             // attr.into_token_stream()
//     //             // return Some(syn::parse2::<Self>(attr.into_token_stream.clone())).transpose()
//     //         }
//     //         // Ignore other attributes
//     //         _ => {},
//     //     }
//     // }

//     match Col::try_from_attributes(&field.attrs) {
//         Ok(col) => {
//             if let Some(col) = col {
//                 Ok(create_with_col(&field_name, &rust_type, has_option, col))
//             } else {
//                 Ok(create_without_col(&field_name, &rust_type, has_option))
//             }
//         }
//         Err(err) => {
//             // if err.
//             Err(Error::new(
//                 std::io::ErrorKind::Other,
//                 format!("解析失败: {}", err),
//             ))
//         }
//     }
// }

// fn create_with_col(
//     field_name: &String,
//     rust_type: &String,
//     has_option: bool,
//     col: Col,
// ) -> TokenStream {
//     // 解析 ignore 字段
//     let ignore = col
//         .ignore
//         .unwrap_or(syn::Lit::Bool(LitBool::new(false, Span::call_site())));
//     // 解析 pk 字段
//     let pk = col
//         .pk
//         .unwrap_or(syn::Lit::Bool(LitBool::new(false, Span::call_site())));
//     // 解析数据库字段名称
//     let has_col_name = col.name.is_some();
//     let col_name = col
//         .name
//         .unwrap_or(syn::Lit::Str(LitStr::new("", Span::call_site())));
//     // 解析自定义数据类型
//     let has_col_type = col.col_type.is_some();
//     let col_type = col
//         .col_type
//         .unwrap_or(syn::Lit::Str(LitStr::new("", Span::call_site())));
//     // 解析 autoincr 字段，是否自增类型
//     let autoincr = col
//         .autoincr
//         .unwrap_or(syn::Lit::Bool(LitBool::new(false, Span::call_site())));

//     let typ = if rust_type == "String" {
//         // 解析字符串长度
//         // 解析 len 值
//         let len_val = parse_size(&col.len);
//         if len_val.is_none() {
//             easy_sqlx_core::sql::schema::types::types::SqlType::new(rust_type, Some(255))
//         } else {
//             easy_sqlx_core::sql::schema::types::types::SqlType::new(rust_type, len_val)
//         }
//     } else {
//         // 解析其它类型
//         easy_sqlx_core::sql::schema::types::types::SqlType::new(rust_type, None)
//     };

//     // 解析说明字段
//     let has_comment = col.comment.is_some();
//     let comment = col
//         .comment
//         .unwrap_or(syn::Lit::Str(LitStr::new("", Span::call_site())));

//     quote!(easy_sqlx_core::sql::schema::column::Column {
//         name: #field_name.to_string(),
//         column: if #has_col_name { Some(#col_name.to_string()) } else { None }, // #col_name.to_string,
//         col_type: if #has_col_type { Some(#col_type.to_string()) } else { None },
//         ignore: #ignore,
//         typ: #typ,
//         // 自增类型 autoincr
//         autoincr: #autoincr,
//         pk: #pk,
//         // 是否可为空 null | not-null
//         nullable: #has_option,
//         comment: if #has_comment { Some(#comment.to_string()) } else { None },
//         default: None,
//     })
// }

// fn create_without_col(field_name: &String, rust_type: &String, has_option: bool) -> TokenStream {
//     // 解析 ignore 字段
//     let ignore = false;

//     let typ = if rust_type == "String" {
//         easy_sqlx_core::sql::schema::types::types::SqlType::new(rust_type, Some(255))
//     } else {
//         // 解析其它类型
//         easy_sqlx_core::sql::schema::types::types::SqlType::new(rust_type, None)
//     };

//     // 解析 pk 字段
//     let pk = false;

//     // 解析字符串类型长度
//     // LitInt:
//     // let str_len = col
//     //     .len
//     //     .unwrap_or(syn::Lit::Int(LitInt::new("255", Span::call_site())));

//     quote!(easy_sqlx_core::sql::schema::column::Column {
//         name: #field_name.to_string(),
//         col_name: None, // #col_name.to_string,
//         col_type: None,
//         ignore: #ignore,
//         typ: #typ,
//         pk: #pk,
//         // 自增类型 autoincr
//         autoincr: false,
//         // 是否可为空 null | not-null
//         nullable: #has_option,
//         comment: None,
//         default: None,
//     })
// }
