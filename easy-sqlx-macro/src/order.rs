use easy_sqlx_core::sql::schema::column::Column;
use proc_macro2::Span;
use quote::quote;

pub fn create_order_func(col: &Column) -> proc_macro2::TokenStream {
    let asc_name = syn::Ident::new(format!("{}_asc", &col.name).as_str(), Span::call_site());
    let desc_name = syn::Ident::new(format!("{}_desc", &col.name).as_str(), Span::call_site());
    let order_name = syn::Ident::new(format!("{}_order", &col.name).as_str(), Span::call_site());
    let field_name = col.name.clone();

    quote! {
        // 升序
        pub fn #asc_name() -> easy_sqlx_core::sql::dialects::page::Order {
            easy_sqlx_core::sql::dialects::page::Order::asc(#field_name.to_string())
        }
        /// 降序
        pub fn #desc_name() -> easy_sqlx_core::sql::dialects::page::Order {
            easy_sqlx_core::sql::dialects::page::Order::desc(#field_name.to_string())
        }
        /// 默认排序
        pub fn #order_name() -> easy_sqlx_core::sql::dialects::page::Order {
            easy_sqlx_core::sql::dialects::page::Order::new(#field_name.to_string())
        }
    }
}