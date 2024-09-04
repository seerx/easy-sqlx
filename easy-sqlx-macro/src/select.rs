use easy_sqlx_core::sql::schema::table::TableSchema;
use proc_macro2::Ident;
use quote::quote;
use syn::Field;

pub fn create_select_by_id(
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
            
            where_args.push(quote! {
                builder = builder.and(#entity::#col_eq(#field_name));
            });
        }
    }
    
    // let args: proc_macro2::TokenStream = id_args.iter().map(|arg| arg.clone()).collect();
    // let note = args.to_string();
    quote! {
        /// 根据 主键 查询
        pub fn select_by_id<'a>(#(#id_args), *) -> easy_sqlx_core::sql::builder::select_builder::SelectBuilder<'a> {
            let mut builder: easy_sqlx_core::sql::builder::select_builder::SelectBuilder<'a> = easy_sqlx_core::sql::builder::select_builder::SelectBuilder::new(Self::table());
            #(#where_args) *
            // }
            builder
        }
    }
}

pub fn create_select_builder() -> proc_macro2::TokenStream {
    quote! {
        pub fn select<'a>() -> easy_sqlx_core::sql::builder::select_builder::SelectBuilder<'a> {
            easy_sqlx_core::sql::builder::select_builder::SelectBuilder::new(Self::table())
        }
    }
}
