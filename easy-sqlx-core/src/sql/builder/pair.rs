use sqlx::{Database, Encode, Type};

pub enum Value {
    Int(i32),
    Long(i64),
    Float(f64),
    Text(String),
}

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


#[derive(Default)]
pub struct Pair<T> 
where
    for<'q> T: 'q + Encode<'q, DB> + Type<DB>,
{
    db: Option<DB>,
    pub name: &'static str,
    pub value: T,
}

 
