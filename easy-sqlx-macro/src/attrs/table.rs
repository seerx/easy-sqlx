use easy_sqlx_core::sql::schema::{index::Index, table::TableSchema};
use syn::{spanned::Spanned, Attribute, Error};

// pub fn check_col_in_table_attrs(attrs: &Vec<Attribute>) -> syn::Result<()> {
//     for attr in attrs.iter() {
//         if attr.path().is_ident("col") {
//             return Err(Error::new(
//                 attr.span(),
//                 format!("Invalid #[col()] on struct"),
//             ));
//         }
//     }
//     Ok(())
// }

pub fn parse_table_attrs(attrs: &Vec<Attribute>, default_table_name: String) -> syn::Result<TableSchema> {
    for attr in attrs.iter() {
        if attr.path().is_ident("col") {
            return Err(Error::new(
                attr.span(),
                format!("Invalid #[col()] on struct"),
            ));
        }
    }

    let mut table = TableSchema::default();

    for attr in attrs.iter() {
        if attr.path().is_ident("table") {
            // 解析 table 属性
            match attr.parse_args::<TableSchema>() {
                Ok(tbl) => {
                    table.assign(tbl)?;
                }
                Err(err) => {
                    return Err(Error::new(err.span(), format!("{err} on table attributes")));
                }
            }
        }
    }

    if table.name.is_empty() {
        table.name = default_table_name;
    }

    // table.comment = Some(format!("1023098308 - {}", table.name));
    let indexes = table.indexes.clone();
    // let ins: Vec<String> = table.indexes.unwrap().iter().map(|i| i.name.clone()).collect();
    // table.comment = Some(format!("1023098308 - {}", ins.join(",")));
    table.indexes = None;
    if let Some(raws) = indexes {
        // table.comment = Some(format!("{} - raw: {}", table.name, raws.len()));
        for idx in &raws {
            table.add_index(idx.clone())?;
        }
    }

    for attr in attrs.iter() {
        if attr.path().is_ident("index") {
            // 解析 index 属性
            match attr.parse_args::<Index>() {
                Ok(idx) => {
                    table.add_index(idx)?;
                }
                Err(err) => {
                    return Err(Error::new(err.span(), format!("{err} on table attributes")));
                }
            }
        }
    }

    // attrs.iter().filter(|attr| attr.path().is_ident(""));

    Ok(table)
}

// pub fn parse_indexes(table: &mut Table, attrs: &Vec<Attribute>) -> syn::Result<Option<Table>> {
//     for attr in attrs.iter() {
//         if attr.path().is_ident("index") {
//             // 解析 index 属性
//             match attr.parse_args::<Index>() {
//                 Ok(idx) => {
//                     table.add_index(idx)?;
//                 }
//                 Err(err) => {
//                     return Err(Error::new(err.span(), format!("{err} on table attributes")));
//                 }
//             }
//         }
//     }

//     // attrs.iter().filter(|attr| attr.path().is_ident(""));

//     Ok(Some(table))
// }
