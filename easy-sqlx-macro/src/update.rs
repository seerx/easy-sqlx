use easy_sqlx_core::sql::schema::table::TableSchema;
use proc_macro2::Ident;
use quote::quote;

pub fn create_update(table: &TableSchema, entity: &Ident) -> proc_macro2::TokenStream {
    // update 绑定参数
    let this = proc_macro2::Ident::new("this", proc_macro2::Span::call_site());
    // let mut insert_bind_args: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut update_args = vec![];
    let mut where_args = vec![];
    for col in table.columns.iter() {
        let field_name = proc_macro2::Ident::new(col.name.as_str(), proc_macro2::Span::call_site()); // &col.name;
        let col_name = &col.get_column_name();

        if col.pk {
            // 主键作为 where 条件
            let col_eq = proc_macro2::Ident::new(format!("{}_eq", &col.name).as_str(), proc_macro2::Span::call_site());
            where_args.push(quote! {
                // let #this = self;
                builder = builder.and(#entity::#col_eq(self.#field_name.clone()));
            });
            // builder.and(User::id_eq(100));
            continue;
        }
        
        if col.nullable {
            update_args.push(quote! {
                // let #this = self;
                builder = builder.set_column(easy_sqlx_core::sql::utils::pair::Pair {
                    name: #col_name.to_string(),
                    value: easy_sqlx_core::sql::utils::value::Value::from(self.#field_name.clone()),
                });
            });
        } else {
            update_args.push(quote! {
                // let #this = self;
                builder = builder.set_column(easy_sqlx_core::sql::utils::pair::Pair {
                    name: #col_name.to_string(),
                    value: easy_sqlx_core::sql::utils::value::Value::from(&self.#field_name.clone()),
                });
            });
        }
    }
    quote! {
        /// 根据主键更新全部数据
        pub fn update<'a>(&self) -> easy_sqlx_core::sql::builder::update_builder::UpdateBuilder<'a> {
            // let table = &Self::table();
            let #this = self;
            let mut builder: easy_sqlx_core::sql::builder::update_builder::UpdateBuilder<'a> = easy_sqlx_core::sql::builder::update_builder::UpdateBuilder::new(Self::table());
            // for col in Self::table().columns {

            // println!("insert 1");
            #(#update_args) *
            #(#where_args) *
            // }
            builder
        }
    }
}

pub fn create_update_builder() -> proc_macro2::TokenStream {
    quote! {
        pub fn build_update<'a>() -> easy_sqlx_core::sql::builder::update_builder::UpdateBuilder<'a> {
            easy_sqlx_core::sql::builder::update_builder::UpdateBuilder::new(Self::table())
        }
    }
}
