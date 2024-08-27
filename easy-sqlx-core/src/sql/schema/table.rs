use easy_sqlx_utils::value_parser::{parse_groups, parse_next, Array};
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Error, Ident, LitStr, Token};

use super::{column::Column, index::Index};

#[derive(Clone, Debug, Default)]
pub struct TableSchema {
    /// 表名称
    pub name: String,
    /// 表所属 schema
    pub schema: Option<String>,
    /// 说明信息
    pub comment: Option<String>,
    /// 索引
    pub indexes: Option<Vec<Index>>,
    /// 列
    pub columns: Vec<Column>,

    /// [控制字段]
    /// 从另外一个表重命名,
    /// 执行重命名的前提条件：1 该表不存在，2 from 指定的表必须存在
    pub from: Option<String>,
    /// [控制字段]
    /// 删除并重建表，给出一个与该表关联的唯一重建标志
    /// 当 重建标志 标志第一次出现时，该表将会重建
    /// 重建会删除表内所有数据，请慎重使用
    pub recreate: Option<String>,
    /// [控制字段]
    /// 删除没有关联 struct 字段的表中的列
    pub trim_columns: bool,
    /// [控制字段]
    /// 删除未定义的索引
    pub trim_indexes: bool,
}

impl TableSchema {
    pub fn name_with_schema(&self) -> String {
        if let Some(schema) = &self.schema {
            return format!("{schema}.{}", self.name.clone());
        }
        self.name.clone()
    }

    pub fn index_name_with_schema(&self, index_name: &String) -> String {
        if let Some(schema) = &self.schema {
            return format!("{schema}.{index_name}");
        }
        index_name.clone()
    }

    /// 检查索引列是否合法
    /// 有效索引列为表中列的字段名称
    pub fn check_indexes_columns(&self) -> syn::Result<()> {
        if let Some(indexes) = self.indexes.as_ref() {
            for index in indexes.iter() {
                for col in index.columns.iter() {
                    if self.find_column(col).is_none() {
                        return Err(Error::new(
                            Span::call_site(),
                            format!("Index's column '{col}' is not exists in table columns"),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn find_column(&self, name: &String) -> Option<Column> {
        self.columns
            .iter()
            .find(|col| {
                if col.column.is_none() {
                    col.name == *name
                } else {
                    col.column.as_ref().unwrap() == name
                }
            })
            .map(|col| (*col).clone())
    }

    /// 合并 source 属性到当前 Table
    pub fn assign(&mut self, source: TableSchema) -> syn::Result<()> {
        if self.name.is_empty() && !source.name.is_empty() {
            self.name = source.name.clone();
        }

        if self.comment.is_none() && !source.comment.is_none() {
            self.comment = source.comment.clone();
        }

        if self.schema.is_none() && !source.schema.is_none() {
            self.schema = source.schema.clone();
        }

        if self.recreate.is_none() {
            self.recreate = source.recreate.clone();
        }
        if self.from.is_none() {
            self.from = source.from.clone();
        }
        if !self.trim_columns {
            self.trim_columns = source.trim_columns;
        }

        if !self.trim_indexes {
            self.trim_indexes = source.trim_indexes;
        }

        if let Some(src_indexes) = source.indexes {
            for idx in src_indexes {
                if self.indexes.is_none() {
                    self.indexes = Some(vec![idx.to_owned()]);
                } else {
                    self.indexes.as_mut().unwrap().push(idx.to_owned());
                }
                // self.add_index(idx.to_owned())?;
            }
        }

        // 列无需复制
        Ok(())
    }

    /// 添加 index，如果 name 冲突则重新命名 name
    pub fn add_index(&mut self, mut index: Index) -> syn::Result<()> {
        // let name = index.get_name();
        let (mut name, setted) = index.get_name(&self.name);
        if self.indexes.is_none() {
            index.name = name;
            self.indexes = Some(vec![index]);
        } else {
            // 获取索引名称
            let mut n = 0;
            loop {
                if self
                    .indexes
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|idx| (*idx).name == name) // 索引列表中的索引名称为已经设置，可以直接对比
                    .is_none()
                {
                    break;
                }
                if setted {
                    // 索引名称是设置的，不允许冲突
                    return Err(syn::Error::new(
                        Span::call_site(),
                        format!("索引名称 {} 冲突", name),
                    ));
                }
                n += 1;
                (name, _) = index.get_name_with_index(&self.name, n);
                // name = format!("{}_{}", index.name.clone(), n).to_string();
            }
            // 设置索引名称
            index.name = name;
            self.indexes.as_mut().unwrap().push(index);
        }

        Ok(())
    }
}

impl ToTokens for TableSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.name.clone();
        let comment = self.comment.clone().unwrap_or("".to_string());
        let has_comment = !comment.is_empty();
        let idxs = self.indexes.clone().unwrap_or(vec![]);
        let has_idxs = !idxs.is_empty();
        let schema = self.schema.clone().unwrap_or("".to_string());
        let has_schema = !schema.is_empty();
        let cols = self.columns.clone();
        let from = self.from.clone().unwrap_or("".to_string());
        let has_from = !from.is_empty();
        let trim_columns = self.trim_columns;
        let trim_indexes = self.trim_indexes;
        let recreate = self.recreate.clone().unwrap_or("".to_string());
        let has_recreate = !recreate.is_empty();
        quote! {
            easy_sqlx_core::sql::schema::table::TableSchema {
                indexes: if #has_idxs { Some([#(#idxs), *].to_vec()) } else { None },
                columns: [#(#cols), *].to_vec(),
                name: #name.to_string(),
                comment: if #has_comment { Some(#comment.to_string()) } else { None },
                schema: if #has_schema { Some(#schema.to_string()) } else { None },
                from: if #has_from { Some(#from.to_string()) } else { None },
                recreate: if #has_recreate { Some(#recreate.to_string()) } else { None },
                trim_columns: #trim_columns,
                trim_indexes: #trim_indexes,
                // raw_indexes: if #has_raw_idxs { Some([#(#raw_idxs), *].to_vec()) } else { None },
            }
        }
        .to_tokens(tokens);
    }
}

impl Parse for TableSchema {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        const EXPECTED_ATTRIBUTE: &str =
            "unexpected attribute, expected any of: name, comment, schema, from, recreate, trim_columns, trim_indexes indexes[]";

        let mut table = TableSchema::default();
        let mut idxes = vec![];
        while !input.is_empty() {
            let ident = input.parse::<Ident>().map_err(|error| {
                Error::new(error.span(), format!("{EXPECTED_ATTRIBUTE}, {error}"))
            })?;
            let attribute = &*ident.to_string();

            match attribute {
                "name" => {
                    table.name = parse_next(input, || input.parse::<LitStr>())
                        .map_err(|err| {
                            Error::new(
                                err.span(),
                                format!("attribute {attribute} parse error, {err}"),
                            )
                        })?
                        .value();
                }
                "from" => {
                    table.from = Some(
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
                "trim_columns" => {
                    table.trim_columns = true;
                }
                "trim_indexes" => {
                    table.trim_indexes = true;
                }
                "recreate" => {
                    table.recreate = Some(
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
                    table.comment = Some(
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
                "schema" => {
                    table.schema = Some(
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
                "indexes" => {
                    // parse_next(input, || input.parse::<LitStr>())
                    //     .map_err(|err| {
                    //         Error::new(
                    //             err.span(),
                    //             format!("attribute {attribute} parse error, {err}"),
                    //         )
                    //     })?
                    //     .value();
                    // parse_punctuated_within_parenthesis::<>(input);
                    // let aaaa = parse_macro_input!(input.to_tokens() with Punctuated::<Index, syn::Token![,]>::parse_terminated);
                    let indexes;
                    syn::bracketed!(indexes in input); // [] 括住 索引

                    // let args_parsed =
                    //     syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
                    //         .parse(input.)
                    //         .unwrap();

                    let a: Array<'static, Index> = parse_groups(&indexes)?;

                    for index in a.iter() {
                        idxes.push(index.clone());
                        // table.add_raw_index(index.clone())
                    }

                    // loop {
                    //     if indexes.is_empty() {
                    //         break;
                    //     }
                    // //     let index =
                    // //         parse_next(input, || indexes.parse::<Index>()).map_err(|err| {
                    // //             Error::new(
                    // //                 err.span(),
                    // //                 format!("attribute {attribute} parse error, {err}"),
                    // //             )
                    // //         })?;
                    // //     idxes.push(index);
                    // //     // let value = parser(input)?;
                    // //     // punctuated.push_value(value);
                    //     if indexes.is_empty() {
                    //         break;
                    //     }
                    //     // let punct = input.parse()?;
                    // //     // punctuated.push_punct(punct);
                    // }
                }
                _ => {
                    return Err(Error::new(ident.span(), EXPECTED_ATTRIBUTE));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        if !idxes.is_empty() {
            // table.raw_indexes = Some(idxes);
            table.indexes = Some(idxes);
        }

        Ok(table)
    }
}
