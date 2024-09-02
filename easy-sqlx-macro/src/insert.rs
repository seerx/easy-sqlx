use easy_sqlx_core::sql::schema::table::TableSchema;
use quote::quote;

pub fn create_insert(table: &TableSchema) -> proc_macro2::TokenStream {
    // insert 绑定参数
    let this = proc_macro2::Ident::new("this", proc_macro2::Span::call_site());
    // let mut insert_bind_args: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut insert_bind_args = vec![];
    for col in table.columns.iter() {
        let field_name = proc_macro2::Ident::new(col.name.as_str(), proc_macro2::Span::call_site()); // &col.name;
        let col_name = &col.get_column_name();
        if col.nullable {
            insert_bind_args.push(quote! {
                let #this = self;
                builder.add_column(easy_sqlx_core::sql::utils::pair::Pair {
                    name: #col_name.to_string(),
                    value: easy_sqlx_core::sql::utils::value::Value::from(self.#field_name),
                });
            });
        } else {
            insert_bind_args.push(quote! {
                let #this = self;
                builder.add_column(easy_sqlx_core::sql::utils::pair::Pair {
                    name: #col_name.to_string(),
                    value: easy_sqlx_core::sql::utils::value::Value::from(&self.#field_name),
                });
            });
        }
    }
    quote! {
        pub fn insert<'a>(&self) -> easy_sqlx_core::sql::builder::easy_insert_builder::InsertBuilder<'a> {
            // let table = &Self::table();

            let mut builder: easy_sqlx_core::sql::builder::easy_insert_builder::InsertBuilder<'a> = easy_sqlx_core::sql::builder::easy_insert_builder::InsertBuilder::new(Self::table());
            // for col in Self::table().columns {

            // println!("insert 1");
            #(#insert_bind_args) *

            // }
            builder
        }
    }
}
