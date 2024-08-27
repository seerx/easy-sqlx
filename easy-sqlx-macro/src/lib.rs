use heck::ToSnakeCase;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attrs;

use attrs::{column::parse_column_attrs, table::parse_table_attrs};

/// 使用示例
/// 定义表结构
/// ```rust,ignore
///     #[derive(Table)]
///     #[table(
///         indexes [
///             (name = "123", columns("a", "b"))
///         ]
///     )]
///     #[index(columns("ooi"))]
///     struct Table1 {
///         // #[col(column = "key", ignore, col_type = "abc", )]
///         #[col(column = "key", comment = "123")]
///         #[col(pk, autoincr, len = 100)]
///         pub id: String,
///         #[col(comment = "姓名", len = 20)]
///         pub name: Option<String>,
///         #[col(ignore)]
///         pub t_o: chrono::NaiveTime,
///     }
/// ```
/// 同步表结构
/// 参数 connection 为数据库连接
/// ```rust,ignore
///  sync_tables(connection, vec![Table1::table()], None).await?;
/// ```
#[proc_macro_derive(Table, attributes(table, index, col))]
pub fn derive_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    // ident 当前枚举名称
    let DeriveInput {
        attrs, ident, data, ..
    } = input;

    // 解析 列 属性
    let mut col_prop_methods: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut cols = Vec::new();
    if let syn::Data::Struct(syn::DataStruct {
        struct_token: _,
        fields,
        semi_token: _,
    }) = data
    {
        for field in fields {
            match parse_column_attrs(&field) {
                Ok(col) => {
                    if let Some(column) = col {
                        // 生成列方法名称
                        let fn_name = syn::Ident::new(
                            format!("col_{}", &column.name).as_str(),
                            Span::call_site(),
                        );
                        // 添加列方法
                        col_prop_methods.push(quote! {
                            pub fn #fn_name() -> &'static str {
                                "123"
                            }
                        });
                        // 添加列
                        cols.push(column);
                    }
                }
                Err(err) => {
                    panic!("{}", err);
                }
            }
        }
    }

    // if let Err(err) = check_col_in_table_attrs(&attrs) {
    //     panic!("{}", err);
    // }

    let default_table_name = ident.clone().to_string().to_snake_case();
    // 解析表属性及索引
    let mut table = parse_table_attrs(&attrs, default_table_name)
        // .map(|opt| {
        //     if let Some(table) = opt {
        //         table
        //     } else {
        //         Table::default()
        //     }
        // })
        .map_err(|err| panic!("{}", err))
        .unwrap();
    // if table.name.is_empty() {
    //     // 如果没有设置 name 属性
    //     table.name = ident.clone().to_string().to_snake_case();
    // }
    table.columns = cols.clone();

    if let Err(err) = table.check_indexes_columns() {
        // 有错误
        panic!("{}", err);
    }

    // 实现 comment 方法
    let output = quote! {
        impl #ident {
            // fn comment(&self) -> &'static str {
            //     match self {
            //         #(#comment_arms),*
            //     }
            // }
            // #(#table_prop_methods) *
            pub fn table() -> easy_sqlx_core::sql::schema::table::TableSchema {
                #table
            }

            #(#col_prop_methods) *

            // fn columns() -> Vec<easy_sqlx_core::sql::schema::column::Column> {
            //     [#(#cols), *].to_vec()
            // }
        }
    };
    output.into()
}
