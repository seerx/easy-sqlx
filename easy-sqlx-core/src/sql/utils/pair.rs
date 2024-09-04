use super::value::Value;
use chrono::NaiveDateTime;
use sqlx::Database;
// impl Value {
//     pub fn get<T: I32, I64, F64, String>(&self) -> T {
//         match self {
//             Self::Int(val) => val,
//             Self::Long(val) => val,
//             Self::Float(v) => v,
//             Self::Text(str) => str,
//         };
//     }
// }

#[derive(Debug, Clone)]
pub struct Pair {
    pub name: String,
    pub value: Value,
}

impl Pair {
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
        self.value.bind_to_query(query)
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
        self.value.bind_to_query_as(query)
    }
}
