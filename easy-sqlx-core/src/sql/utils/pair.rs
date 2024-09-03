#[cfg(feature = "postgres")]
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

#[derive(Debug, Clone)]
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
        self.value.bind_to_query(query)
    }
}
