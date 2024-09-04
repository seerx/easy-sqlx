use crate::sql::utils::{pair::Pair, quote::Quoter};
use chrono::NaiveDateTime;
use sqlx::Database;

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Eq,        // =
    Neq,       // <>
    Gt,        // >
    Ge,        // >=
    Lt,        // <
    Le,        // <=
    In,        // in
    Like,      // like
    IsNull,    // is null
    IsNotNull, // is not null
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Eq => "eq".to_string(),
            Operator::Neq => "neq".to_string(),
            Operator::Gt => "gt".to_string(),
            Operator::Ge => "ge".to_string(),
            Operator::Lt => "lt".to_string(),
            Operator::Le => "le".to_string(),
            Operator::In => "in".to_string(),
            Operator::Like => "like".to_string(),
            Operator::IsNull => "is_null".to_string(),
            Operator::IsNotNull => "is_not_null".to_string(),
        }
    }
}

impl Operator {
    pub fn resolve(name: String) -> Self {
        match name.as_str() {
            "eq" => Self::Eq,
            "neq" => Self::Neq,
            "gt" => Self::Gt,
            "ge" => Self::Ge,
            "lt" => Self::Lt,
            "le" => Self::Le,
            "in" => Self::In,
            "like" => Self::Like,
            "is_null" => Self::IsNull,
            "is_not_null" => Self::IsNotNull,
            _ => Self::Eq,
        }
    }

    pub fn is_not_param(&self) -> bool {
        match self {
            Self::IsNull | Self::IsNotNull => true,
            _ => false,
        }
    }
    // pub fn name(&self) -> String {
    //     match self {
    //         Operator::Eq => "Eq".to_string(),
    //         Operator::Neq => "Neq".to_string(),
    //         Operator::Gt => "Gt".to_string(),
    //         Operator::Ge => "Ge".to_string(),
    //         Operator::Lt => "Lt".to_string(),
    //         Operator::Le => "Le".to_string(),
    //         Operator::In => "In".to_string(),
    //         Operator::Like => "Like".to_string(),
    //         Operator::Is => "Is".to_string(),
    //         Operator::IsNot => "IsNot".to_string(),
    //     }
    // }
    pub fn sql(&self) -> String {
        match self {
            Operator::Eq => "=".to_string(),
            Operator::Neq => "<>".to_string(),
            Operator::Gt => ">".to_string(),
            Operator::Ge => ">=".to_string(),
            Operator::Lt => "<".to_string(),
            Operator::Le => "<=".to_string(),
            Operator::In => "in".to_string(),
            Operator::Like => "like".to_string(),
            Operator::IsNull => "is null".to_string(),
            Operator::IsNotNull => "is not null".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Condition {
    Condition(Pair, Operator),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

impl Condition {
    pub fn is_condition(&self) -> bool {
        match self {
            Condition::Condition(_, _) => true,
            Condition::And(_, _) => false,
            Condition::Or(_, _) => false,
        }
    }

    pub fn is_and(&self) -> bool {
        match self {
            Condition::Condition(_, _) => false,
            Condition::And(_, _) => true,
            Condition::Or(_, _) => false,
        }
    }

    pub fn is_or(&self) -> bool {
        match self {
            Condition::Condition(_, _) => false,
            Condition::And(_, _) => false,
            Condition::Or(_, _) => true,
        }
    }
}

impl Condition {
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
        match self {
            Condition::Condition(p, o) => {
                if o.is_not_param() {
                    query
                } else {
                    p.bind_to_query(query)
                }
            }
            Condition::And(left, right) => {
                let qry = left.bind_to_query(query);
                right.bind_to_query(qry)
            }
            Condition::Or(left, right) => {
                let qry = left.bind_to_query(query);
                right.bind_to_query(qry)
            }
        }
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
        match self {
            Condition::Condition(p, o) => {
                if o.is_not_param() {
                    query
                } else {
                    p.bind_to_query_as(query)
                }
            }
            Condition::And(left, right) => {
                let qry = left.bind_to_query_as(query);
                right.bind_to_query_as(qry)
            }
            Condition::Or(left, right) => {
                let qry = left.bind_to_query_as(query);
                right.bind_to_query_as(qry)
            }
        }
    }

    pub fn sql(&self, param_index: usize, quoter: &Quoter) -> (String, usize) {
        match self {
            Condition::Condition(p, o) => {
                let field = p.name.clone();
                let op = o.sql();

                if o.is_not_param() {
                    // 不需要参数
                    return (format!("{} {op}", quoter.quote(&field)), param_index);
                }

                if *o == Operator::In {
                    // in 操作
                    // vec![0; p.value.get_len()].iter().map(|n| );
                    let mut params = vec![];
                    for n in 0..p.value.len() {
                        params.push(format!("${}", param_index + n));
                    }

                    return (
                        format!("{} {op} ({})", quoter.quote(&field), params.join(",")),
                        param_index + p.value.len(),
                    );
                }
                (
                    format!("{} {op} ${param_index}", quoter.quote(&field)),
                    param_index + 1,
                )
            }
            Condition::And(left, right) => {
                let (left_cond, index) = left.sql(param_index, quoter);
                let (right_cond, index) = right.sql(index, quoter);
                if left.is_or() {
                    if right.is_or() {
                        (format!("({left_cond}) and ({right_cond})"), index)
                    } else {
                        (format!("({left_cond}) and {right_cond}"), index)
                    }
                } else {
                    if right.is_or() {
                        (format!("{left_cond} and ({right_cond})"), index)
                    } else {
                        (format!("{left_cond} and {right_cond}"), index)
                    }
                }
            }
            Condition::Or(left, right) => {
                let (left_cond, index) = left.sql(param_index, quoter);
                let (right_cond, index) = right.sql(index, quoter);
                if left.is_and() {
                    if right.is_and() {
                        (format!("({left_cond}) or ({right_cond})"), index)
                    } else {
                        (format!("({left_cond}) or {right_cond}"), index)
                    }
                } else {
                    if right.is_and() {
                        (format!("{left_cond} or ({right_cond})"), index)
                    } else {
                        (format!("{left_cond} or {right_cond}"), index)
                    }
                }
            }
        }
    }
}

pub trait WhereAppend<T> {
    fn and(self, cond: T) -> Self;
    fn or(self, cond: T) -> Self;
}

#[derive(Default, Debug, Clone)]
pub struct Where {
    cond: Option<Box<Condition>>,
    // params: Vec<Value>,
}

impl WhereAppend<Condition> for Where {
    fn and(mut self, cond: Condition) -> Self {
        if let Some(c) = self.cond {
            self.cond = Some(Box::new(Condition::And(c, Box::new(cond))));
        } else {
            self.cond = Some(Box::new(cond));
        }
        self
    }

    fn or(mut self, cond: Condition) -> Self {
        if let Some(c) = self.cond {
            self.cond = Some(Box::new(Condition::Or(c, Box::new(cond))));
        } else {
            self.cond = Some(Box::new(cond));
        }
        self
    }
}

impl WhereAppend<Where> for Where {
    fn and(mut self, w: Where) -> Self {
        if let Some(wcond) = w.cond {
            if let Some(c) = self.cond {
                self.cond = Some(Box::new(Condition::And(c, wcond)));
            } else {
                self.cond = Some(wcond);
            }
        }
        self
    }

    fn or(mut self, w: Where) -> Self {
        if let Some(wcond) = w.cond {
            if let Some(c) = self.cond {
                self.cond = Some(Box::new(Condition::Or(c, wcond)));
            } else {
                self.cond = Some(wcond);
            }
        }
        self
    }
}

impl From<Condition> for Where {
    fn from(cond: Condition) -> Self {
        Self {
            cond: Some(Box::new(cond)),
        }
    }
}

impl Where {
    pub fn new(cond: Condition) -> Self {
        Self {
            cond: Some(Box::new(cond)),
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
        if let Some(c) = &self.cond {
            return c.bind_to_query(query);
        }
        query
    }

    pub fn bind_to_query_as<'a, O, DB: Database>(
        &self,
        query: sqlx::query::QueryAs<'a, DB, O, DB::Arguments<'a>>,
    ) -> sqlx::query::QueryAs<'a, DB, O, DB::Arguments<'a>>
    where
        O: std::marker::Send,
        O: Unpin,
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
        if let Some(c) = &self.cond {
            return c.bind_to_query_as(query);
        }
        query
    }

    pub fn sql(&self, param_index: usize, quoter: &Quoter) -> (String, usize) {
        if let Some(cond) = &self.cond {
            cond.sql(param_index, quoter)
        } else {
            ("".to_string(), param_index)
        }
    }
}
