use easy_sqlx_core::sql::schema::column::Column;
use proc_macro2::Span;
use quote::quote;
use syn::{Field, Type};

pub fn create_field_wrapper(
    col: &Column,
    field: &Field,
    syn_type: &Type,
) -> Vec<proc_macro2::TokenStream> {
    let field_name = proc_macro2::Ident::new(col.name.as_str(), proc_macro2::Span::call_site()); // &col.name;
    let col_name = &col.get_column_name();
    let ty = &field.ty;

    let mut wrappers = vec![];
    // wrappers.push(quote! {
    //     pub fn #field_name(val: #ty) -> easy_sqlx_core::sql::utils::pair::Pair {
    //         easy_sqlx_core::sql::utils::pair::Pair {
    //             name: #col_name.to_string(),
    //             value: easy_sqlx_core::sql::utils::value::Value::from(val)
    //         }
    //     }
    // });

    let array_pair = syn::Ident::new(format!("{}_array", &field_name).as_str(), Span::call_site());
    wrappers.push(quote! {
        pub fn #array_pair(val: Vec<#syn_type>) -> easy_sqlx_core::sql::utils::pair::Pair {
            easy_sqlx_core::sql::utils::pair::Pair {
                name: #col_name.to_string(),
                value: easy_sqlx_core::sql::utils::value::Value::from(val)
            }
        }
    });
    if col.nullable {
        wrappers.push(quote! {
            pub fn #field_name(val: #syn_type) -> easy_sqlx_core::sql::utils::pair::Pair {
                easy_sqlx_core::sql::utils::pair::Pair {
                    name: #col_name.to_string(),
                    value: easy_sqlx_core::sql::utils::value::Value::from(val)
                }
            }
        });
        let fd2 = syn::Ident::new(format!("{}_opt", &field_name).as_str(), Span::call_site());
        wrappers.push(quote! {
            pub fn #fd2(val: #ty) -> easy_sqlx_core::sql::utils::pair::Pair {
                easy_sqlx_core::sql::utils::pair::Pair {
                    name: #col_name.to_string(),
                    value: easy_sqlx_core::sql::utils::value::Value::from(val)
                }
            }
        });
    } else {
        wrappers.push(quote! {
            pub fn #field_name(val: #ty) -> easy_sqlx_core::sql::utils::pair::Pair {
                easy_sqlx_core::sql::utils::pair::Pair {
                    name: #col_name.to_string(),
                    value: easy_sqlx_core::sql::utils::value::Value::from(val)
                }
            }
        });
    }

    wrappers
}
