use easy_sqlx_core::sql::schema::table::TableSchema;
use proc_macro2::Ident;
use quote::quote;
use syn::Field;

pub fn create_delete(table: &TableSchema, entity: &Ident) -> proc_macro2::TokenStream {
    // 绑定参数
    let this = proc_macro2::Ident::new("this", proc_macro2::Span::call_site());

    let mut where_args = vec![];
    for col in table.columns.iter() {
        let field_name = proc_macro2::Ident::new(col.name.as_str(), proc_macro2::Span::call_site());
        if col.pk {
            // 主键作为 where 条件
            let col_eq = proc_macro2::Ident::new(
                format!("{}_eq", &col.name).as_str(),
                proc_macro2::Span::call_site(),
            );

            if col.nullable {
                // 该条件应该不会生效
                where_args.push(quote! {
                    if let Some(v) = self.#field_name.clone() {
                        builder = builder.and(#entity::#col_eq(v));
                    }
                });
            } else {
                where_args.push(quote! {
                    builder = builder.and(#entity::#col_eq(self.#field_name.clone()));
                });
            }
        }
    }
    quote! {
        /// 根据主键删除记录
        pub fn delete<'a>(&self) -> easy_sqlx_core::sql::builder::delete_builder::DeleteBuilder<'a> {
            // let table = &Self::table();
            let #this = self;
            let mut builder: easy_sqlx_core::sql::builder::delete_builder::DeleteBuilder<'a> = easy_sqlx_core::sql::builder::delete_builder::DeleteBuilder::new(Self::table());
            #(#where_args) *
            // }
            builder
        }
    }
}


pub fn create_delete_by_id(
    table: &TableSchema,
    entity: &Ident,
    struct_fields: &Vec<Field>,
) -> proc_macro2::TokenStream {
    // 绑定参数
    let mut id_args = vec![];
    let mut where_args = vec![];
    for (n, col) in table.columns.iter().enumerate() {
        let field_name = proc_macro2::Ident::new(col.name.as_str(), proc_macro2::Span::call_site());
        if col.pk {
            let field_type = &struct_fields[n].ty;
            // let field_name = proc_macro2::Ident::new(col.name.as_str(), proc_macro2::Span::call_site());
            id_args.push(quote! {
                #field_name: #field_type
            });

            // 主键作为 where 条件
            let col_eq = proc_macro2::Ident::new(
                format!("{}_eq", &col.name).as_str(),
                proc_macro2::Span::call_site(),
            );

            if col.nullable {
                // 该条件应该不会生效
                where_args.push(quote! {
                    if let Some(v) = self.#field_name.clone() {
                        builder = builder.and(#entity::#col_eq(v));
                    }
                });
            } else {
                where_args.push(quote! {
                    builder = builder.and(#entity::#col_eq(#field_name));
                });
            }
        }
    }

    // let args: proc_macro2::TokenStream = id_args.iter().map(|arg| arg.clone()).collect();
    // let note = args.to_string();
    quote! {
        /// 根据 主键 查询
        pub fn delete_by_id<'a>(#(#id_args), *) -> easy_sqlx_core::sql::builder::delete_builder::DeleteBuilder<'a> {
            let mut builder: easy_sqlx_core::sql::builder::delete_builder::DeleteBuilder<'a> = easy_sqlx_core::sql::builder::delete_builder::DeleteBuilder::new(Self::table());
            #(#where_args) *
            // }
            builder
        }
    }
}


pub fn create_delete_builder() -> proc_macro2::TokenStream {
    quote! {
        pub fn build_delete<'a>() -> easy_sqlx_core::sql::builder::delete_builder::DeleteBuilder<'a> {
            easy_sqlx_core::sql::builder::delete_builder::DeleteBuilder::new(Self::table())
        }
    }
}
