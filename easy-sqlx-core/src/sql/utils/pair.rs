use sqlx::{postgres::PgArguments, Postgres};

use super::value::Value;

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

#[derive(Debug)]
pub struct Pair {
    pub name: String,
    pub value: Value,
}

impl Pair {
    #[cfg(feature = "postgres")]
    pub fn bind_to_query<'a>(
        &self,
        query: sqlx::query::Query<'a, Postgres, PgArguments>,
    ) -> sqlx::query::Query<'a, Postgres, PgArguments> {
        return match &self.value {
            Value::Int(val) => query.bind(*val),
            Value::Long(val) => query.bind(*val),
            Value::Double(val) => query.bind(*val),
            Value::ChronoDate(val) => query.bind(*val),
            Value::Text(val) => query.bind(val.clone()),
            Value::Byte(val) => query.bind(*val),
            Value::Short(val) => query.bind(*val),
            Value::Float(val) => query.bind(*val),
        };
        // query = query.bind(0i32);
        // query
    }
}
