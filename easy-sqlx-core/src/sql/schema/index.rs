use easy_sqlx_utils::{ternary, value_parser::parse_next};
use quote::{quote, ToTokens};
use syn::{parse::Parse, punctuated::Punctuated, token::Comma, Error, Ident, LitStr, Token};

#[derive(Clone, Debug, Default)]
pub struct Index {
    pub columns: Vec<String>,
    pub name: String,
    // pub regular: bool,
    pub unique: bool,
}

impl Index {
    pub fn is_name_equal(&self, idx: &Index) -> bool {
        self.name.to_uppercase() == idx.name.to_uppercase()
    }

    pub fn is_columns_equal(&self, idx: &Index) -> bool {
        idx.columns == self.columns
    }

    pub fn get_name(&self, table_name: &String) -> (String, bool) {
        let type_name = ternary!(self.unique, "uqe", "idx");
        if self.name.is_empty() {
            let name = self.columns.join("_");
            (format!("{table_name}_{type_name}_{name}"), false)
        } else {
            (format!("{table_name}_{type_name}_{}", self.name), true)
        }
    }

    pub fn get_name_with_index(&self, table_name: &String, index: i32) -> (String, bool) {
        let type_name = ternary!(self.unique, "uqe", "idx");
        if self.name.is_empty() {
            let name = self.columns.join("_");
            // 自动生成的索引名称，可以添加序号
            (format!("{table_name}_{type_name}_{index}_{name}"), false)
        } else {
            // 手动设置名称的索引不允许添加序号
            (format!("{table_name}_{type_name}_{}", self.name), true)
        }
    }

    // pub fn generate_name(&mut self) {
    //     if self.name.is_empty() {
    //         let mut name = self.columns.join("_");
    //         name.push_str("_index");
    //         self.name = name.to_owned();
    //     }
    // }
    // pub fn get_name(self) -> String {
    //     return self.name.clone().unwrap();
    // }
}

impl ToTokens for Index {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.name.clone(); // .unwrap_or("".to_string());
        let unique = self.unique;

        let cols = self.columns.clone();
        quote! {
            easy_sqlx_core::sql::schema::index::Index {
                columns: [#(#cols.to_string()), *].to_vec(),
                name: #name.to_string(),
                unique: #unique,
                ..Default::default()
            }
        }
        .to_tokens(tokens);
    }
}

impl Parse for Index {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        const EXPECTED_ATTRIBUTE: &str =
            "unexpected attribute, expected any of: name, columns[], unique";

        let mut index = Index::default();
        while !input.is_empty() {
            let ident = input.parse::<Ident>().map_err(|error| {
                Error::new(error.span(), format!("{EXPECTED_ATTRIBUTE}, {error}"))
            })?;
            let attribute = &*ident.to_string();

            match attribute {
                "name" => {
                    index.name = parse_next(input, || input.parse::<LitStr>())
                        .map_err(|err| {
                            Error::new(
                                err.span(),
                                format!("attribute {attribute} parse error, {err}"),
                            )
                        })?
                        .value();
                }
                "columns" => {
                    let columns;
                    syn::parenthesized!(columns in input); // () 括住 索引字段列表

                    let scopes = Punctuated::<LitStr, Comma>::parse_terminated(&columns)?
                        .iter()
                        .map(LitStr::value)
                        .collect::<Vec<_>>();

                    index.columns = scopes;
                }
                "unique" => {
                    index.unique = true;
                }
                _ => {
                    return Err(Error::new(ident.span(), EXPECTED_ATTRIBUTE));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        // index.generate_name();
        Ok(index)
    }
}
