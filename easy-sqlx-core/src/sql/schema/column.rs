use easy_sqlx_utils::value_parser::parse_next;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Error, Ident, LitInt, LitStr, Token};
// use tools::macros::value_parser::parse_next;

use super::types::types::SqlType;

#[derive(Clone, Debug, Default)]
pub struct Column {
    /// 结构体字段名称
    pub name: String,

    /// 数据库字段名称
    pub column: Option<String>,
    /// 数据库字段的数据类型，用于没有合适的类型时，可以自定义
    pub col_type: Option<String>,
    /// 数据库数据类型
    pub typ: SqlType,

    /// [控制字段]
    /// 是否忽略该字段，
    /// 在解析定义时，如果是 true 则不向列表中添加该列
    pub ignore: bool,
    /// 主键 pk
    pub pk: bool,
    /// 自增类型 autoincr
    pub autoincr: bool,
    /// 说明信息
    pub comment: Option<String>,

    /// 是否可为空 null | not-null
    pub nullable: bool,

    // 默认值 default(值)
    pub default: Option<String>,

    /// [控制字段]
    /// 从另外一个字段重命名,
    /// 执行重命名的前提条件：1 该字段不存在，2 from 指定的字段必须存在
    /// 重命名后如果字段属性不一致，则执行修改操作
    pub from: Option<String>,

    /// [控制字段]
    /// 如果字段更新失败， replace 为 true 时，则删除旧字段并添加新字段
    /// 注意：删除旧字段会连带字段中的数据一起删除，并且不能恢复
    pub replace: bool,
    // /// [代码生成控制]
    // /// 数据类型使用的 Option 数量
    // pub rust_type_options: isize,
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.is_name_equal(other)
        // self.name == other.name
            // && self.column == other.column
            // && self.col_type == other.col_type
            && self.typ == other.typ
            // && self.ignore == other.ignore
            // && self.pk == other.pk // 主键不参与对比的原因是，不允许更改主键
            && self.autoincr == other.autoincr
            // && self.comment == other.comment
            && self.nullable == other.nullable
            && self.default == other.default
    }
}

impl Eq for Column {}

impl Column {
    pub fn is_name_equal(&self, col: &Column) -> bool {
        if let Some(s_col) = &self.column {
            if let Some(o_col) = &col.column {
                s_col.to_uppercase() == o_col.to_uppercase()
            } else {
                s_col.to_uppercase() == col.name.to_uppercase()
            }
        } else {
            if let Some(o_col) = &col.column {
                self.name.to_uppercase() == o_col.to_uppercase()
            } else {
                self.name.to_uppercase() == col.name.to_uppercase()
            }
        }
    }

    pub fn get_column_name(&self) -> String {
        if let Some(name) = self.column.clone() {
            name
        } else {
            self.name.clone()
        }
    }

    pub fn get_query_column_name(&self) -> String {
        if let Some(name) = self.column.clone() {
            format!("{name} as {}", self.name)
        } else {
            self.name.clone()
        }
    }

    pub fn assign(&mut self, source: &Column) {
        if !self.pk {
            self.pk = source.pk;
        }
        if !self.autoincr {
            self.autoincr = source.autoincr;
        }
        if self.column.is_none() && source.column.is_some() {
            self.column = source.column.clone();
        }
        if self.col_type.is_none() && source.col_type.is_some() {
            self.col_type = source.col_type.clone();
        }
        if self.comment.is_none() && source.comment.is_some() {
            self.comment = source.comment.clone();
        }
        if self.default.is_none() && source.default.is_some() {
            self.default = source.default.clone();
        }
        if self.from.is_some() {
            self.from = source.from.clone();
        }
        if !self.replace {
            self.replace = source.replace;
        }
        // self.typ = source.typ.clone();
    }
}

impl ToTokens for Column {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ignore = self.ignore;
        let name = self.name.clone();
        let col_name = self.column.clone().unwrap_or("".to_string());
        let has_col_name = !col_name.is_empty();

        // let mut has_col_type = self.col_type.is_some();
        let col_type = self.col_type.clone().unwrap_or("".to_string());
        let has_col_type = !col_type.is_empty();
        let typ = self.typ.to_token_stream();
        let pk = self.pk;
        let autoincr = self.autoincr;
        let nullable = self.nullable;
        // TokenStream::new().a
        // let mut has_comment = self.comment.is_some();
        let comment = self.comment.clone().unwrap_or("".to_string());
        let has_comment = !comment.is_empty();

        let default = self.default.clone().unwrap_or("".to_string());
        let has_default = !default.is_empty();

        let from = self.from.clone().unwrap_or("".to_string());
        let has_from = !from.is_empty();

        let replace = self.replace;

        // let col_type =
        // let has_len2 = self.len2.is_some();
        // let len2 = self.len2.unwrap_or(0);
        // let len = self.len.clone();
        quote! {
            easy_sqlx_core::sql::schema::column::Column {
                ignore: #ignore,
                pk: #pk,
                autoincr: #autoincr,
                nullable: #nullable,
                name: #name.to_string(),
                column: if #has_col_name { Some(#col_name.to_string()) } else { None },
                col_type: if #has_col_type { Some(#col_type.to_string()) } else { None },
                typ: #typ,
                comment: if #has_comment { Some(#comment.to_string()) } else { None },
                default: if #has_default { Some(#default.to_string()) } else { None },
                from: if #has_from { Some(#from.to_string()) } else { None },
                replace: #replace,
                // typ:
                // len2: if #has_len2 { Some(#len2) } else { None },
                ..Default::default()
            }
        }
        .to_tokens(tokens);
    }
}

// pub fn parse_next<T: FnOnce() -> Result<R, syn::Error>, R: Sized>(
//     input: ParseStream,
//     next: T,
// ) -> Result<R, syn::Error> {
//     input.parse::<Token![=]>()?;
//     next()
// }

impl Parse for Column {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        const EXPECTED_ATTRIBUTE: &str =
            "unexpected attribute, expected any of: ignore, pk, column, col_type, autoincr, comment, default, from, replace";

        let mut column = Column::default();

        while !input.is_empty() {
            let ident = input.parse::<Ident>().map_err(|error| {
                Error::new(error.span(), format!("{EXPECTED_ATTRIBUTE}, {error}"))
            })?;
            let attribute = &*ident.to_string();

            // for attr in attrs {
            //     match attr.path().get_ident() {
            //         Some(ident) if ident == #attr_name => {
            //             return Some(syn::parse2::<Self>(attr.into_token_stream())).transpose()
            //         }
            //         // Ignore other attributes
            //         _ => {},
            //     }
            // }

            match attribute {
                "ignore" => {
                    column.ignore = true;
                }
                "pk" => {
                    column.pk = true;
                }
                "autoincr" => {
                    column.autoincr = true;
                }
                "col_type" => {
                    column.col_type = Some(
                        parse_next(input, || input.parse::<LitStr>())
                            .map_err(|err| {
                                Error::new(
                                    err.span(),
                                    format!("attribute {attribute} parse error, {err}"),
                                )
                            })?
                            .value(),
                    );
                }
                "from" => {
                    column.from = Some(
                        parse_next(input, || input.parse::<LitStr>())
                            .map_err(|err| {
                                Error::new(
                                    err.span(),
                                    format!("attribute {attribute} parse error, {err}"),
                                )
                            })?
                            .value(),
                    );
                }
                "replace" => {
                    column.replace = true;
                }
                "len" => {
                    column.typ.len = Some(
                        parse_next(input, || input.parse::<LitInt>())
                            .map_err(|err| {
                                Error::new(
                                    err.span(),
                                    format!("attribute {attribute} parse error, {err}"),
                                )
                            })?
                            .base10_digits()
                            .parse()
                            .map_err(|err| {
                                Error::new(
                                    Span::call_site(),
                                    format!("attribute {attribute} parse error, {err}"),
                                )
                            })?,
                        // .value(),
                    );
                }
                "column" => {
                    // let info_stream;
                    // parenthesized!(info_stream in input);
                    // column.column = parse_string(info_stream, "", "").map(|str| Some(str))?;
                    // parse_next(input, || {
                    //     Ok(())
                    // });
                    // input.parse::<Token![=]>();
                    // parse_next(input, || Ok(()))?;
                    column.column = Some(
                        parse_next(input, || input.parse::<LitStr>())
                            .map_err(|err| {
                                Error::new(
                                    err.span(),
                                    format!("attribute {attribute} parse error, {err}"),
                                )
                            })?
                            .value(),
                    );
                }
                "comment" => {
                    column.comment = Some(
                        parse_next(input, || input.parse::<LitStr>())
                            .map_err(|err| {
                                Error::new(
                                    err.span(),
                                    format!("attribute {attribute} parse error, {err}"),
                                )
                            })?
                            .value(),
                    );
                }
                _ => {
                    return Err(Error::new(ident.span(), EXPECTED_ATTRIBUTE));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(column)
    }
}
