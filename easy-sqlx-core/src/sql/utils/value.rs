#[derive(Debug, Clone)]
pub enum Value {
    Bool(Option<bool>),
    // Byte(Option<i8>),
    Binary(Option<Vec<u8>>),
    Short(Option<i16>),
    Int(Option<i32>),
    Long(Option<i64>),
    Float(Option<f32>),
    Double(Option<f64>),
    Text(Option<String>),
    ChronoDate(Option<chrono::NaiveDateTime>),

    Array(Vec<Self>),
}

use chrono::NaiveDateTime;
use sqlx::Database;

impl Value {
    pub fn len(&self) -> usize {
        match self {
            Value::Array(ary) => ary.len(),
            _ => 1,
        }
    }

    pub fn bind_to_query<'a, DB: Database>(
        &self,
        query: sqlx::query::Query<'a, DB, DB::Arguments<'a>>,
    ) -> sqlx::query::Query<'a, DB, DB::Arguments<'a>>
    where
        Option<bool>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        bool: sqlx::Type<DB>,
        Option<i16>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        i16: sqlx::Type<DB>,
        Option<i32>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        i32: sqlx::Type<DB>,
        Option<i64>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        i64: sqlx::Type<DB>,
        Option<f64>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        f64: sqlx::Type<DB>,
        Option<f32>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        f32: sqlx::Type<DB>,
        Option<NaiveDateTime>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        NaiveDateTime: sqlx::Type<DB>,
        Option<String>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        String: sqlx::Type<DB>,
        Option<Vec<u8>>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        Vec<u8>: sqlx::Type<DB>,
    {
        return match self {
            Value::Int(val) => query.bind(*val),
            Value::Long(val) => query.bind(*val),
            Value::Double(val) => query.bind(*val),
            Value::ChronoDate(val) => query.bind(*val),
            Value::Text(val) => query.bind(val.clone()),
            Value::Binary(val) => query.bind(val.clone()),
            Value::Short(val) => query.bind(*val),
            Value::Float(val) => query.bind(*val),
            Value::Bool(val) => query.bind(*val),
            Value::Array(ary) => {
                let mut qry = query;
                if !ary.is_empty() {
                    for val in ary {
                        qry = val.bind_to_query(qry);
                    }
                }
                qry
            }
        };
    }

    pub fn bind_to_query_as<'a, O, DB: Database>(
        &self,
        query: sqlx::query::QueryAs<'a, DB, O, DB::Arguments<'a>>,
    ) -> sqlx::query::QueryAs<'a, DB, O, DB::Arguments<'a>>
    where
        Option<bool>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        bool: sqlx::Type<DB>,
        Option<i16>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        i16: sqlx::Type<DB>,
        Option<i32>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        i32: sqlx::Type<DB>,
        Option<i64>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        i64: sqlx::Type<DB>,
        Option<f64>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        f64: sqlx::Type<DB>,
        Option<f32>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        f32: sqlx::Type<DB>,
        Option<NaiveDateTime>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        NaiveDateTime: sqlx::Type<DB>,
        Option<String>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        String: sqlx::Type<DB>,
        Option<Vec<u8>>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        Vec<u8>: sqlx::Type<DB>,
    {
        return match self {
            Value::Int(val) => query.bind(*val),
            Value::Long(val) => query.bind(*val),
            Value::Double(val) => query.bind(*val),
            Value::ChronoDate(val) => query.bind(*val),
            Value::Text(val) => query.bind(val.clone()),
            Value::Binary(val) => query.bind(val.clone()),
            Value::Short(val) => query.bind(*val),
            Value::Float(val) => query.bind(*val),
            Value::Bool(val) => query.bind(*val),
            Value::Array(ary) => {
                let mut qry = query;
                if !ary.is_empty() {
                    for val in ary {
                        qry = val.bind_to_query_as(qry);
                    }
                }
                qry
            }
        };
    }

    pub fn bind_to_query_scalar<'a, O, DB: Database>(
        &self,
        query: sqlx::query::QueryScalar<'a, DB, O, DB::Arguments<'a>>,
    ) -> sqlx::query::QueryScalar<'a, DB, O, DB::Arguments<'a>>
    where
        Option<bool>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        bool: sqlx::Type<DB>,
        Option<i16>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        i16: sqlx::Type<DB>,
        Option<i32>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        i32: sqlx::Type<DB>,
        Option<i64>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        i64: sqlx::Type<DB>,
        Option<f64>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        f64: sqlx::Type<DB>,
        Option<f32>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        f32: sqlx::Type<DB>,
        Option<NaiveDateTime>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        NaiveDateTime: sqlx::Type<DB>,
        Option<String>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        String: sqlx::Type<DB>,
        Option<Vec<u8>>: sqlx::Encode<'a, DB> + sqlx::Decode<'a, DB>,
        Vec<u8>: sqlx::Type<DB>,
    {
        return match self {
            Value::Int(val) => query.bind(*val),
            Value::Long(val) => query.bind(*val),
            Value::Double(val) => query.bind(*val),
            Value::ChronoDate(val) => query.bind(*val),
            Value::Text(val) => query.bind(val.clone()),
            Value::Binary(val) => query.bind(val.clone()),
            Value::Short(val) => query.bind(*val),
            Value::Float(val) => query.bind(*val),
            Value::Bool(val) => query.bind(*val),
            Value::Array(ary) => {
                let mut qry = query;
                if !ary.is_empty() {
                    for val in ary {
                        qry = val.bind_to_query_scalar(qry);
                    }
                }
                qry
            }
        };
    }
}

// impl From<Vec<u8>> for Value {
//     fn from(value: Vec<u8>) -> Self {
//         Self::Binary(value)
//     }
// }

macro_rules! impl_from_num_for_value {
    ($t:ty, $v:ident) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Value::$v(Some(value))
            }
        }

        impl From<&$t> for Value {
            fn from(value: &$t) -> Self {
                Value::$v(Some(*value))
            }
        }

        impl From<Option<$t>> for Value {
            fn from(value: Option<$t>) -> Self {
                Value::$v(value)
            }
        }
    };
}

macro_rules! impl_from_unsigned_num_for_value {
    ($t:ty, $v:ident, $target:ty) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Value::$v(Some(value as $target))
            }
        }

        impl From<&$t> for Value {
            fn from(value: &$t) -> Self {
                Value::$v(Some(*value as $target))
            }
        }

        impl From<Option<$t>> for Value {
            fn from(value: Option<$t>) -> Self {
                if let Some(val) = value {
                    Value::$v(Some(val as $target))
                } else {
                    Value::$v(None)
                }
            }
        }
    };
}

macro_rules! impl_from_clone_for_value {
    ($t:ty, $v:ident) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Value::$v(Some(value.to_owned()))
            }
        }

        impl From<&$t> for Value {
            fn from(value: &$t) -> Self {
                Value::$v(Some(value.clone()))
            }
        }

        impl From<Option<$t>> for Value {
            fn from(value: Option<$t>) -> Self {
                Value::$v(value.clone())
            }
        }
    };
}

impl_from_num_for_value!(bool, Bool);

impl_from_num_for_value!(i16, Short);
impl_from_num_for_value!(i32, Int);
impl_from_num_for_value!(i64, Long);
// impl_from_unsigned_num_for_value!(u8, Byte, i8);
impl_from_unsigned_num_for_value!(u16, Short, i16);
impl_from_unsigned_num_for_value!(u32, Int, i32);
impl_from_unsigned_num_for_value!(u64, Long, i64);
impl_from_clone_for_value!(Vec<u8>, Binary);
impl_from_num_for_value!(f64, Double);
impl_from_num_for_value!(f32, Float);
impl_from_clone_for_value!(chrono::NaiveDateTime, ChronoDate);
impl_from_clone_for_value!(String, Text);

// impl_from_array_for_value!($($t:ty) *);

// impl From<Vec<i64>> for Value {
//     fn from(value: Vec<i64>) -> Self {
//         Self::Array(value.iter().map(|v| Value::from(v)).collect())
//     }
// }

macro_rules! impl_from_array_for_value {
    ( $( $t:ty) * ) => {
        $(
            impl From<Vec<$t>> for Value {
                fn from(value: Vec<$t>) -> Self {
                    Self::Array(value.iter().map(|v| Value::from(v)).collect())
                }
            }
        )*
    };
}
impl_from_array_for_value!(bool i16 i32 i64 u16 u32 u64 f32 f64 String chrono::NaiveDateTime);

// impl From<Option<i32>> for Value {
//     fn from(value: Option<i32>) -> Self {
//         Self::Int(value)
//     }
// }

// impl From<&i64> for Value {
//     fn from(value: &i64) -> Self {
//         Self::Long(*value)
//     }
// }

// impl From<&f64> for Value {
//     fn from(value: &f64) -> Self {
//         Self::Float(*value)
//     }
// }

// impl From<&String> for Value {
//     fn from(value: &String) -> Self {
//         Self::Text(value.clone())
//     }
// }

// impl From<&chrono::NaiveDateTime> for Value {
//     fn from(value: &chrono::NaiveDateTime) -> Self {
//         Self::ChronoDate(value.clone())
//     }
// }
