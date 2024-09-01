use std::io;
 

use easy_sqlx_utils::ternary;

use crate::sql::schema::{column::Column, index::Index};

use super::quote::{ always_reserve, Quoter};

const DEFAULT_SCHEMA: &str = "public";

pub struct Context {
    default_schema: String,
    quoter: Quoter,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            default_schema: DEFAULT_SCHEMA.to_owned(),
            #[cfg(feature = "postgres")]
            quoter: Quoter::new(b'\"', b'\"', always_reserve),
        }
    }
}

impl Context {
    pub fn new(schema: String) -> Self {
        let mut s = Self::default();
        if !schema.is_empty() {
            s.default_schema = schema;
        }
        s
    }
    // pub fn with_schema(default_schema: String) -> Self {
    //     let mut ctx = Self::default();
    //     ctx.default_schema = default_schema;
    //     ctx
    // }

    pub fn quote(&self, str: &String) -> String {
        self.quoter.quote(str)
    }

    pub fn get_default_schema(&self) -> String {
        self.default_schema.clone()
    }

    pub fn table_name_with_schema(&self, name: &String) -> String {
        if !self.default_schema.is_empty() && !name.contains(".") {
            return format!("{}.{}", self.default_schema, name);
        }
        name.clone()
    }

    pub fn is_table_name_equal(&self, table1: &String, table2: &String) -> bool {
        self.table_name_with_schema(table1).to_uppercase()
            == self.table_name_with_schema(table2).to_uppercase()
    }

    pub fn sql_column(
        &self,
        col: &Column,
        inline_pk: bool,
        auto_incr_str: Option<&str>,
        fn_sql_type: fn(col: &Column) -> String,
    ) -> String {
        let mut sql = String::new();
        // 字段名称
        self.quoter.quote_to(&mut sql, &col.get_column_name());
        // if err := bd.Quoter().QuoteTo(&sql, col.FieldName()); err != nil {
        // 	return "", err
        // }
        sql.push(' ');
        // if err := sql.WriteByte(' '); err != nil {
        // 	return "", err
        // }

        // 数据类型
        let sql_type = fn_sql_type(col);
        sql.push_str(&sql_type);
        sql.push(' ');
        // sqlType := bd.SQLType(col)

        // if _, err := sql.WriteString(sqlType); err != nil {
        // 	return "", err
        // }
        // if err := sql.WriteByte(' '); err != nil {
        // 	return "", err
        // }

        if inline_pk && col.pk {
            // 只有一个字段是主键，且该字段是主键
            sql.push_str("PRIMARY KEY ");
            // if _, err := sql.WriteString("PRIMARY KEY "); err != nil {
            // 	return "", err
            // }

            // if col.IsAutoIncrement {

            // 	if _, err := sql.WriteString(bd.AutoIncrStr()); err != nil {
            // 		return "", err
            // 	}
            // 	if err := sql.WriteByte(' '); err != nil {
            // 		return "", err
            // 	}
            // }
        }

        if col.autoincr {
            // 该字段是自增类型
            if let Some(incr) = auto_incr_str {
                sql.push_str(incr);
                sql.push(' ');
            }
        }

        if let Some(def) = &col.default {
            sql.push_str("DEFAULT ");
            sql.push_str(&def.to_string());
            sql.push_str(" ");
            // if col.typ.name == sql_types::VARCHAR
            //     || (col.typ.name == sql_types::CHAR && col.typ.fixed_len.is_none())
            //     || col.typ.name == sql_types::TEXT
            // {
            //     // 文本类型
            //     // 这里需要改代码
            //     sql.push_str("DEFAULT ('') ");
            // } else {
            //     sql.push_str("DEFAULT ");
            //     sql.push_str(&def.to_string());
            //     sql.push_str(" ");
            // }
        }

        // else if col.Default != "" {
        // 	if _, err := sql.WriteString("DEFAULT "); err != nil {
        // 		return "", err
        // 	}
        // 	if _, err := sql.WriteString(col.Default); err != nil {
        // 		return "", err
        // 	}
        // 	if err := sql.WriteByte(' '); err != nil {
        // 		return "", err
        // 	}
        // }

        if col.nullable {
            sql.push_str("NULL ");
        } else {
            sql.push_str("NOT NULL ");
        }

        // if col.Nullable {
        // 	if _, err := sql.WriteString("NULL "); err != nil {
        // 		return "", err
        // 	}
        // } else {
        // 	if _, err := sql.WriteString("NOT NULL "); err != nil {
        // 		return "", err
        // 	}
        // }
        sql
        // return sql.String(), nil
    }

    pub fn sql_add_column(
        &self,
        table_name: &String,
        column: &Column,
        auto_incr_str: Option<&str>,
        fn_sql_type: fn(col: &Column) -> String,
    ) -> String {
        let col_def = self.sql_column(column, false, auto_incr_str, fn_sql_type);
        format!(
            "ALTER TABLE {} ADD COLUMN {col_def}",
            self.quote(&self.table_name_with_schema(table_name))
        )
    }

    pub fn sql_drop_column(&self, table_name: &String, column_name: &String) -> String {
        format!(
            "ALTER TABLE {} DROP COLUMN IF EXISTS {}",
            self.quote(&self.table_name_with_schema(table_name)),
            self.quote(column_name)
        )
    }

    pub fn sql_alter_column(
        &self,
        table_name: &String,
        old: &Column,
        new: &Column,
        fn_sql_type: fn(col: &Column) -> String,
        ignore_default: bool,
    ) -> io::Result<Vec<String>> {
        let table = self.quote(&self.table_name_with_schema(table_name));
        let column = old.get_column_name();
        let mut sqls = vec![];

        if fn_sql_type(old) != fn_sql_type(new) {
            // 类型发生变化
            if new.typ.name == old.typ.name {
                // 类型名称没有变化，大概是字符串长度发生变化
                // ALTER TABLE table_name ALTER COLUMN column_name TYPE character_type(length);
                if let Some(len) = new.typ.len {
                    sqls.push(format!(
                        "ALTER TABLE {table} ALTER COLUMN {column} TYPE character_type({len})"
                    ));
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("字符字段必须有长度: {column} on {table}"),
                    ));
                }
            } else {
                // ALTER TABLE table_name ALTER COLUMN column_name TYPE new_data_type;
                let sql_type = fn_sql_type(new);
                sqls.push(format!(
                    "ALTER TABLE {table} ALTER COLUMN {column} TYPE {sql_type}"
                ));
            }
        }
        if !ignore_default {
            if old.default != new.default {
                if let Some(def) = &new.default {
                    // ALTER TABLE table_name ALTER COLUMN column_name SET DEFAULT default_value;
                    sqls.push(format!(
                        "ALTER TABLE {table} ALTER COLUMN {column} SET DEFAULT {def}"
                    ));
                } else {
                    // ALTER TABLE table_name ALTER COLUMN column_name DROP DEFAULT;
                    sqls.push(format!(
                        "ALTER TABLE {table} ALTER COLUMN {column} DROP DEFAULT"
                    ));
                }
            }
        }
        if old.nullable != new.nullable {
            if new.nullable {
                // ALTER TABLE table_name ALTER COLUMN column_name DROP NOT NULL;
                sqls.push(format!(
                    "ALTER TABLE {table} ALTER COLUMN {column} DROP NOT NULL"
                ));
            } else {
                // ALTER TABLE table_name ALTER COLUMN column_name SET NOT NULL;
                sqls.push(format!(
                    "ALTER TABLE {table} ALTER COLUMN {column} SET NOT NULL"
                ));
            }
        }

        // let col_def = self.sql_column(column, false, auto_incr_str);
        // format!(
        //     "ALTER TABLE {} ALTER COLUMN {col_def}",
        //     self.quote(&self.table_name_with_schema(table_name))
        // )
        Ok(sqls)
    }

    pub fn sql_drop_table(&self, table_name: &String) -> String {
        format!(
            "DROP TABLE IF EXISTS {}",
            self.quote(&self.table_name_with_schema(table_name))
        )
    }

    pub fn sql_create_index(&self, table_name: &String, index: &Index) -> Option<String> {
        if index.columns.is_empty() {
            return None;
        }
        let unique = ternary!(index.unique, " UNIQUE", ""); // if index.unique { " UNIQUE" } else { "" };
        Some(format!(
            "CREATE{unique} INDEX {} ON {} ({})",
            self.quote(&index.name),
            self.quote(&self.table_name_with_schema(table_name)),
            index.columns.join(",")
        ))
    }

    pub fn sql_drop_index(&self, index_name_with_schema: &String) -> String {
        format!(
            "DROP INDEX IF EXISTS {}",
            self.quote(&self.table_name_with_schema(index_name_with_schema))
        )
    }
}
