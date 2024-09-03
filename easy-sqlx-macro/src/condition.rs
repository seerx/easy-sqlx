use easy_sqlx_core::sql::{dialects::condition::Operator, schema::column::Column};
use proc_macro2::Span;
use quote::quote;
use syn::{Field, Type};

fn create_condition_func(
    col_name: &String,
    field_name: &String,
    syn_type: &Type,
    op: Operator,
) -> proc_macro2::TokenStream {
    let oper = op.to_string();
    let in_name = syn::Ident::new(
        format!("{}_{}", field_name, &oper).as_str(),
        Span::call_site(),
    );
    if op.is_not_param() {
        return quote! {
            pub fn #in_name() -> easy_sqlx_core::sql::dialects::condition::Condition {
                let val: Option<#syn_type> = None;
                let pair = easy_sqlx_core::sql::utils::pair::Pair {
                    name: #col_name.to_string(),
                    value: easy_sqlx_core::sql::utils::value::Value::from(val)
                };
                let op = easy_sqlx_core::sql::dialects::condition::Operator::resolve(#oper.to_string());
                easy_sqlx_core::sql::dialects::condition::Condition::Condition(pair, op)
            }
        }
    }
    quote! {
        pub fn #in_name(val: #syn_type) -> easy_sqlx_core::sql::dialects::condition::Condition {
            let pair = easy_sqlx_core::sql::utils::pair::Pair {
                name: #col_name.to_string(),
                value: easy_sqlx_core::sql::utils::value::Value::from(val)
            };
            let op = easy_sqlx_core::sql::dialects::condition::Operator::resolve(#oper.to_string());
            easy_sqlx_core::sql::dialects::condition::Condition::Condition(pair, op)
        }
    }
}

pub fn create_conditions(
    col: &Column,
    field: &Field,
    syn_type: &Type,
    rust_type: String,
) -> Vec<proc_macro2::TokenStream> {
    let field_name = proc_macro2::Ident::new(col.name.as_str(), proc_macro2::Span::call_site()); // &col.name;
    let col_name = &col.get_column_name();
    let ty = &field.ty;

    let mut conditions = vec![];

    // in 操作
    let in_name = syn::Ident::new(format!("{}_in", &field_name).as_str(), Span::call_site());
    conditions.push(quote! {
        pub fn #in_name(val: Vec<#syn_type>) -> easy_sqlx_core::sql::dialects::condition::Condition {
            let pair = easy_sqlx_core::sql::utils::pair::Pair {
                name: #col_name.to_string(),
                value: easy_sqlx_core::sql::utils::value::Value::from(val)
            };
            easy_sqlx_core::sql::dialects::condition::Condition::Condition(pair, easy_sqlx_core::sql::dialects::condition::Operator::In)
        }
    });

    conditions.push(create_condition_func(
        &col_name,
        &col.name,
        syn_type,
        Operator::Eq,
    ));
    conditions.push(create_condition_func(
        &col_name,
        &col.name,
        syn_type,
        Operator::Neq,
    ));

    conditions.push(create_condition_func(
        &col_name,
        &col.name,
        syn_type,
        Operator::Lt,
    ));
    conditions.push(create_condition_func(
        &col_name,
        &col.name,
        syn_type,
        Operator::Le,
    ));

    conditions.push(create_condition_func(
        &col_name,
        &col.name,
        syn_type,
        Operator::Gt,
    ));
    conditions.push(create_condition_func(
        &col_name,
        &col.name,
        syn_type,
        Operator::Ge,
    ));
    if rust_type == "String" {
        conditions.push(create_condition_func(
            &col_name,
            &col.name,
            syn_type,
            Operator::Like,
        ));
    }
    // pub fn and_like(self, p: Pair) -> Self {
    //     self.and_operator(p, Operator::Like)
    // }

    if col.nullable {
        conditions.push(create_condition_func(
            &col_name,
            &col.name,
            syn_type,
            Operator::IsNull,
        ));
        conditions.push(create_condition_func(
            &col_name,
            &col.name,
            syn_type,
            Operator::IsNotNull,
        ));
        // pub fn and_is(self, p: Pair) -> Self {
        //     self.and_operator(p, Operator::Is)
        // }
        // pub fn and_is_not(self, p: Pair) -> Self {
        //     self.and_operator(p, Operator::IsNot)
        // }
        // conditions.push(quote! {
        //     pub fn #field_name(val: #syn_type) -> easy_sqlx_core::sql::utils::pair::Pair {
        //         easy_sqlx_core::sql::utils::pair::Pair {
        //             name: #col_name.to_string(),
        //             value: easy_sqlx_core::sql::utils::value::Value::from(val)
        //         }
        //     }
        // });
        // let fd2 = syn::Ident::new(format!("{}2", &field_name).as_str(), Span::call_site());
        // conditions.push(quote! {
        //     pub fn #fd2(val: #ty) -> easy_sqlx_core::sql::utils::pair::Pair {
        //         easy_sqlx_core::sql::utils::pair::Pair {
        //             name: #col_name.to_string(),
        //             value: easy_sqlx_core::sql::utils::value::Value::from(val)
        //         }
        //     }
        // });
    }

    conditions
}
