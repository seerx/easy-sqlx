use condition::create_conditions;
use delete::{create_delete, create_delete_builder};
use field::create_field_wrapper;
use heck::ToSnakeCase;
use insert::{create_insert, create_insert_builder};
use order::create_order_func;
use proc_macro2::Span;
use quote::quote;
use select::{create_select_builder, create_select_by_id};
use syn::{parse_macro_input, DeriveInput};

mod attrs;
mod condition;
mod delete;
mod field;
mod insert;
mod update;
mod select;
mod order;

use attrs::{column::parse_column_attrs, table::parse_table_attrs};
use update::{create_update, create_update_builder};

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
///         pub blob: Vec<u8>,
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

    // 列名称函数
    let mut col_name_methods: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut col_names: Vec<String> = Vec::new();
    let mut cols = Vec::new();

    // 列属性函数
    let mut col_wrapper_methods: Vec<proc_macro2::TokenStream> = Vec::new();
    // 条件属性函数
    let mut col_conditions: Vec<proc_macro2::TokenStream> = Vec::new();
    // 排序生成
    let mut col_order_methods: Vec<proc_macro2::TokenStream> = Vec::new();

    let mut struct_fields: Vec<syn::Field> = vec![];

    if let syn::Data::Struct(syn::DataStruct {
        struct_token: _,
        fields,
        semi_token: _,
    }) = data
    {
        for field in fields {
            match parse_column_attrs(&field) {
                Ok((col, rust_type, syn_type, is_vec)) => {
                    if let Some(column) = col {
                        if let Some(rust_type) = rust_type {
                            if let Some(syn_type) = syn_type {
                                let field_name = &column.name;
                                // 生成列方法名称
                                let fn_name = syn::Ident::new(
                                    format!("col_{}", &field_name).as_str(),
                                    Span::call_site(),
                                );
                                let col_name = column.get_column_name();
                                col_names.push(col_name.clone());
                                // 添加列方法
                                col_name_methods.push(quote! {
                                    /// #col_name 列名称
                                    pub fn #fn_name() -> &'static str {
                                        #col_name
                                    }
                                });

                                // 生成列函数
                                let wrappers =
                                    create_field_wrapper(&column, &field, syn_type, is_vec);
                                col_wrapper_methods.extend(wrappers);

                                // 生成条件属性函数
                                let conds =
                                    create_conditions(&column, &field, syn_type, rust_type, is_vec);
                                col_conditions.extend(conds);

                                // 生成排序函数
                                col_order_methods.push(create_order_func(&column));

                                // 储存字段
                                struct_fields.push(field);
                           

                                // let self_dot_name = syn::Ident::new(
                                //     format!("$self.{}", &column.name).as_str(),
                                //     Span::call_site(),
                                // );

                                // 添加列
                                cols.push(column);
                            }
                        }
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
        .map_err(|err| panic!("{}", err))
        .unwrap();
    table.columns = cols.clone();

    if let Err(err) = table.check_indexes_columns() {
        // 有错误
        panic!("{}", err);
    }

    let table_name = table.name_with_schema();

    let insert = create_insert(&table);
    let build_insert = create_insert_builder();

    let update = create_update(&table, &ident);
    let build_update = create_update_builder();

    let delete = create_delete(&table, &ident);
    let build_delete = create_delete_builder();

    let build_select = create_select_builder();
    let select_by_id = create_select_by_id(&table, &ident, &struct_fields);

    // 实现 comment 方法
    let output = quote! {
        use easy_sqlx_core::sql::dialects::condition::WhereAppend;
        
        impl #ident {
            /// 获取数据库表名称
            pub fn table_name() -> &'static str {
                #table_name
            }

            /// 获取表结构定义
            pub fn table() -> easy_sqlx_core::sql::schema::table::TableSchema {
                #table
            }

            /// 列名称函数
            #(#col_name_methods) *
            /// 获取所有列名称
            pub fn all_cols() -> Vec<&'static str> {
                [#(#col_names), *].to_vec()
            }

            #insert
            #build_insert

            #update
            #build_update

            #delete
            #build_delete

            #(#col_order_methods) *

            #build_select
            #select_by_id

            #(#col_wrapper_methods) *

            #(#col_conditions) *
            // fn columns() -> Vec<easy_sqlx_core::sql::schema::column::Column> {
            //     [#(#cols), *].to_vec()
            // }
        }
    };
    output.into()
}
