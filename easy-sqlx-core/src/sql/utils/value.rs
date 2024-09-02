#[derive(Debug)]
pub enum Value {
    Bool(Option<bool>),
    Byte(Option<i8>),
    Short(Option<i16>),
    Int(Option<i32>),
    Long(Option<i64>),

    LongArray(Vec<i64>),
    // UByte(Option<u8>),
    // UShort(Option<u16>),
    // UInt(Option<u32>),
    // ULong(Option<u64>),
    Float(Option<f32>),
    Double(Option<f64>),
    Text(Option<String>),
    ChronoDate(Option<chrono::NaiveDateTime>),
}

impl Value {
    pub fn get_len(&self) -> usize {
        match self {
            Value::LongArray(ary) => ary.len(),
            _ => 1,
        }
    } 
}

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

macro_rules! impl_from_array_for_value {
    ($t:ty, $v:ident) => {
        impl From<Vec<$t>> for Value {
            fn from(value: Vec<$t>) -> Self {
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
impl_from_num_for_value!(i8, Byte);
impl_from_num_for_value!(i16, Short);
impl_from_num_for_value!(i32, Int);
impl_from_num_for_value!(i64, Long); 
impl_from_unsigned_num_for_value!(u8, Byte, i8);
impl_from_unsigned_num_for_value!(u16, Short, i16);
impl_from_unsigned_num_for_value!(u32, Int, i32);
impl_from_unsigned_num_for_value!(u64, Long, i64);
impl_from_num_for_value!(f64, Double);
impl_from_num_for_value!(f32, Float);
impl_from_clone_for_value!(chrono::NaiveDateTime, ChronoDate);
impl_from_clone_for_value!(String, Text);

impl_from_array_for_value!(i64, LongArray);
 
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
