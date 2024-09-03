use easy_sqlx_core::sql::schema::table::TableSchema;
use quote::quote;

pub fn create_update(table: &TableSchema) -> proc_macro2::TokenStream {
    // insert 绑定参数
    let this = proc_macro2::Ident::new("this", proc_macro2::Span::call_site());
    // let mut insert_bind_args: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut update_args = vec![];
    for col in table.columns.iter() {
        if col.pk {
            // 主键不更新
            continue;
        }
        let field_name = proc_macro2::Ident::new(col.name.as_str(), proc_macro2::Span::call_site()); // &col.name;
        let col_name = &col.get_column_name();
        if col.nullable {
            update_args.push(quote! {
                // let #this = self;
                builder = builder.set_column(easy_sqlx_core::sql::utils::pair::Pair {
                    name: #col_name.to_string(),
                    value: easy_sqlx_core::sql::utils::value::Value::from(self.#field_name),
                });
            });
        } else {
            update_args.push(quote! {
                // let #this = self;
                builder = builder.set_column(easy_sqlx_core::sql::utils::pair::Pair {
                    name: #col_name.to_string(),
                    value: easy_sqlx_core::sql::utils::value::Value::from(&self.#field_name),
                });
            });
        }
    }
    quote! {
        pub fn update<'a>(&self) -> easy_sqlx_core::sql::builder::update_builder::UpdateBuilder<'a> {
            // let table = &Self::table();
            let #this = self;
            let mut builder: easy_sqlx_core::sql::builder::update_builder::UpdateBuilder<'a> = easy_sqlx_core::sql::builder::update_builder::UpdateBuilder::new(Self::table());
            // for col in Self::table().columns {

            // println!("insert 1");
            #(#update_args) *

            // }
            builder
        }
    }
}

pub fn create_update_builder() -> proc_macro2::TokenStream {
    quote! {
        pub fn build_update<'a>() -> easy_sqlx_core::sql::builder::update_builder::UpdateBuilder<'a> {
            easy_sqlx_core::sql::builder::update_builder::UpdateBuilder::new(Self::table());
        }
    }
}
