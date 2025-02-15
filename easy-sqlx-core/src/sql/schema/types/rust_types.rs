pub const R_BOOL: &str = "bool";
pub const R_CHAR: &str = "char";
pub const R_U8: &str = "u8";
pub const R_I8: &str = "i8";
pub const R_U16: &str = "u16";
pub const R_I16: &str = "i16";
pub const R_U32: &str = "u32";
pub const R_I32: &str = "i32";
pub const R_U64: &str = "u64";
pub const R_I64: &str = "i64";
pub const R_F16: &str = "f16";
pub const R_F32: &str = "f32";
pub const R_F64: &str = "f64";
pub const R_STRING: &str = "String";
pub const R_BINARY: &str = "Vec<u8>";
#[cfg(feature="chrono")] 
pub const R_CHRONO_DATE: &str = "NaiveDate";
#[cfg(feature="chrono")]
pub const R_CHRONO_DATE_FULL: &str = "chrono::NaiveDate";
#[cfg(feature="chrono")] 
pub const R_CHRONO_DATETIME: &str = "NaiveDateTime";
#[cfg(feature="chrono")]
pub const R_CHRONO_DATETIME_FULL: &str = "chrono::NaiveDateTime";
#[cfg(feature="chrono")] 
pub const R_CHRONO_TIME: &str = "NaiveTime";
#[cfg(feature="chrono")]
pub const R_CHRONO_TIME_FULL: &str = "chrono::NaiveTime";

// }