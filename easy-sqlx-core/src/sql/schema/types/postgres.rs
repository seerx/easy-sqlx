use super::sql_types;
use pilota::lazy_static::lazy_static;

use crate::sql::schema::{
    column::Column,
    types::{
        rust_types::{
            R_BOOL, R_CHAR, R_CHRONO_DATE, R_CHRONO_DATETIME, R_CHRONO_DATETIME_FULL,
            R_CHRONO_DATE_FULL, R_CHRONO_TIME, R_CHRONO_TIME_FULL, R_F32, R_F64, R_I16, R_I32,
            R_I64, R_I8, R_STRING, R_U16, R_U32, R_U64, R_U8,
        },
        types::TypeRelation,
    },
};

// #[cfg(feature = "postgres")]
lazy_static! {
    pub static ref TYPE_REPLATIONS: Vec<TypeRelation> = vec![
        TypeRelation {
            rust: R_BOOL,
            sql: sql_types::BOOLEAN,
            maybe_types: Some(vec![sql_types::BOOL]),
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_U8,
            sql: super::sql_types::SMALL_INT,
            maybe_types: Some(vec![super::sql_types::SMALL_SERIAL]),
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_I8,
            sql: super::sql_types::SMALL_INT,
            maybe_types: Some(vec![super::sql_types::SMALL_SERIAL]),
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_CHAR,
            sql: super::sql_types::CHAR,
            maybe_types: Some(vec![super::sql_types::CHARACTER]),
            fix_len: Some(1),
            default_len: None,
        },
        TypeRelation {
            rust: R_I16,
            sql: super::sql_types::SMALL_INT,
            maybe_types: Some(vec![super::sql_types::SMALL_SERIAL]),
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_U16,
            sql: super::sql_types::SMALL_INT,
            maybe_types: Some(vec![super::sql_types::SMALL_SERIAL]),
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_I32,
            sql: super::sql_types::INT,
            maybe_types: Some(vec![super::sql_types::INTEGER, super::sql_types::SERIAL]),
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_U32,
            sql: super::sql_types::INT,
            maybe_types: Some(vec![super::sql_types::INTEGER, super::sql_types::SERIAL]),
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_I64,
            sql: super::sql_types::BIG_INT,
            maybe_types: Some(vec![super::sql_types::BIG_SERIAL]),
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_U64,
            sql: super::sql_types::BIG_INT,
            maybe_types: Some(vec![super::sql_types::BIG_SERIAL]),
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_F32,
            sql: super::sql_types::REAL,
            maybe_types: None,
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_F64,
            sql: super::sql_types::DOUBLE,
            maybe_types: Some(vec![super::sql_types::DOUBLE_PRECISION]),
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_STRING,
            sql: super::sql_types::VARCHAR,
            maybe_types: Some(vec![super::sql_types::CHARACTER_VARYING]),
            fix_len: None,
            default_len: Some(255),
        },
        TypeRelation {
            rust: R_CHRONO_DATE,
            sql: super::sql_types::DATE,
            maybe_types: Some(vec![super::sql_types::TIMESTAMP_WITHOUT_TIME_ZONE]),
            fix_len: None,
            default_len: None,
        },
        TypeRelation {
            rust: R_CHRONO_DATE_FULL,
            sql: super::sql_types::DATE,
            maybe_types: Some(vec![super::sql_types::TIMESTAMP_WITHOUT_TIME_ZONE]),
            fix_len: None,
            default_len: Some(6),
        },
        TypeRelation {
            rust: R_CHRONO_DATETIME,
            sql: super::sql_types::TIME_STAMP,
            maybe_types: Some(vec![super::sql_types::TIME_STAMP]),
            fix_len: None,
            default_len: Some(6),
        },
        TypeRelation {
            rust: R_CHRONO_DATETIME_FULL,
            sql: super::sql_types::TIME_STAMP,
            maybe_types: Some(vec![super::sql_types::TIME_STAMP]),
            fix_len: None,
            default_len: Some(6),
        },
        TypeRelation {
            rust: R_CHRONO_TIME,
            sql: super::sql_types::TIME,
            maybe_types: Some(vec![super::sql_types::TIME_WITHOUT_TIME_ZONE]),
            fix_len: None,
            default_len: Some(6),
        },
        TypeRelation {
            rust: R_CHRONO_TIME_FULL,
            sql: super::sql_types::TIME,
            maybe_types: Some(vec![super::sql_types::TIME_WITHOUT_TIME_ZONE]),
            fix_len: None,
            default_len: Some(6),
        }
    ];
}

/// 转换为 sql 类型
pub fn convert_sql_type(col: &Column) -> String {
    let mut sql_typ = if let Some(typ) = &col.col_type {
        typ.into()
    } else {
        col.typ.clone()
    };

    if sql_typ.name == sql_types::TINY_INT {
        // TINY_INT 转为 SMALL_INT
        sql_typ.name = sql_types::SMALL_INT.to_string();
    }
    if sql_typ.name == sql_types::NCHAR {
        sql_typ.name = sql_types::CHAR.to_string();
    }
    if sql_typ.name == sql_types::NVARCHAR {
        sql_typ.name = sql_types::VARCHAR.to_string();
    }

    let sql_type_name = sql_typ.name.as_str();

    match sql_type_name {
        sql_types::BIT => sql_types::BOOLEAN.to_string(),
        sql_types::SMALL_INT => {
            // 短整形
            if col.autoincr {
                // 自增类型
                sql_types::SMALL_SERIAL.to_string()
            } else {
                sql_types::SMALL_INT.to_string()
            }
        }
        sql_types::BIG_INT => {
            // 长整形
            if col.autoincr {
                // 自增类型
                sql_types::BIG_SERIAL.to_string()
            } else {
                sql_types::BIG_INT.to_string()
            }
        }
        sql_types::INT | sql_types::MEDIUM_INT | sql_types::INTEGER => {
            // 整形
            if col.autoincr {
                // 自增类型
                sql_types::SERIAL.to_string()
            } else {
                sql_types::INT.to_string()
            }
        }
        sql_types::FLOAT => sql_types::REAL.to_string(),
        sql_types::DOUBLE => sql_types::DOUBLE_PRECISION.to_string(),
        sql_types::DATE_TIME => {
            sql_typ.to_string()
            // if sql_typ. sql_types::TIME_STAMP.to_string();
        }
        sql_types::TIME_STAMPZ => sql_types::TIMESTAMP_WITH_TIME_ZONE.to_string(),
        sql_types::TINY_TEXT | sql_types::MEDIUM_TEXT | sql_types::LONG_TEXT => {
            sql_types::TEXT.to_string()
        }
        sql_types::BLOB | sql_types::TINY_BLOB | sql_types::MEDIUM_BLOB | sql_types::LONG_BLOB => {
            sql_types::BYTEA.to_string()
        }
        sql_types::CHAR => {
            if let Some(len) = sql_typ.len {
                format!("{}({})", sql_types::CHAR, len)
            } else {
                format!("{}({})", sql_types::CHAR, sql_typ.fixed_len.unwrap_or(255))
            }
        }
        sql_types::VARCHAR => {
            format!("{}({})", sql_types::VARCHAR, sql_typ.len.unwrap_or(255))
        }
        _ => {
            if let Some(len) = sql_typ.len {
                if let Some(len2) = sql_typ.len2 {
                    return format!("{}({}, {})", sql_type_name, len, len2);
                }
                return format!("{}({})", sql_type_name, len);
            }
            sql_type_name.to_string()
        }
    }

    // sql_typ.name.as_str()
    // ""
}

// pub struct NameRelation {
//     pub rust: &'static str,
//     pub sql: Vec<&'static str>,
// }

// pub const AA: NameRelation = NameRelation {
//     rust: "i32",
//     sql: [INT].as_vec(),
// };

// // 索引类型
// pub const (
// 	IndexType = iota + 1
// 	UniqueType
// )

// pub const (
// 	UNKNOW_TYPE = iota
// 	TEXT_TYPE
// 	BLOB_TYPE
// 	TIME_TYPE
// 	NUMERIC_TYPE
// 	ARRAY_TYPE
// )

// var SqlTypes = map[string]int{
// 	Bit:       NUMERIC_TYPE,
// 	TinyInt:   NUMERIC_TYPE,
// 	SmallInt:  NUMERIC_TYPE,
// 	MediumInt: NUMERIC_TYPE,
// 	Int:       NUMERIC_TYPE,
// 	Integer:   NUMERIC_TYPE,
// 	BigInt:    NUMERIC_TYPE,

// 	Enum:  TEXT_TYPE,
// 	Set:   TEXT_TYPE,
// 	Json:  TEXT_TYPE,
// 	Jsonb: TEXT_TYPE,

// 	XML: TEXT_TYPE,

// 	Char:       TEXT_TYPE,
// 	NChar:      TEXT_TYPE,
// 	Varchar:    TEXT_TYPE,
// 	NVarchar:   TEXT_TYPE,
// 	TinyText:   TEXT_TYPE,
// 	Text:       TEXT_TYPE,
// 	NText:      TEXT_TYPE,
// 	MediumText: TEXT_TYPE,
// 	LongText:   TEXT_TYPE,
// 	Uuid:       TEXT_TYPE,
// 	Clob:       TEXT_TYPE,
// 	SysName:    TEXT_TYPE,

// 	Date:          TIME_TYPE,
// 	DateTime:      TIME_TYPE,
// 	Time:          TIME_TYPE,
// 	TimeStamp:     TIME_TYPE,
// 	TimeStampz:    TIME_TYPE,
// 	SmallDateTime: TIME_TYPE,
// 	Year:          TIME_TYPE,

// 	Decimal:    NUMERIC_TYPE,
// 	Numeric:    NUMERIC_TYPE,
// 	Real:       NUMERIC_TYPE,
// 	Float:      NUMERIC_TYPE,
// 	Double:     NUMERIC_TYPE,
// 	Money:      NUMERIC_TYPE,
// 	SmallMoney: NUMERIC_TYPE,

// 	Binary:    BLOB_TYPE,
// 	VarBinary: BLOB_TYPE,

// 	TinyBlob:         BLOB_TYPE,
// 	Blob:             BLOB_TYPE,
// 	MediumBlob:       BLOB_TYPE,
// 	LongBlob:         BLOB_TYPE,
// 	Bytea:            BLOB_TYPE,
// 	UniqueIdentifier: BLOB_TYPE,

// 	Bool: NUMERIC_TYPE,

// 	Serial:    NUMERIC_TYPE,
// 	BigSerial: NUMERIC_TYPE,

// 	Array: ARRAY_TYPE,
// }
