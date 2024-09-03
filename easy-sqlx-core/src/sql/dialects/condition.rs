use crate::sql::utils::pair::Pair;

#[derive(PartialEq)]
pub enum Operator {
    Eq,    // =
    Neq,   // <>
    Gt,    // >
    Ge,    // >=
    Lt,    // <
    Le,    // <=
    In,    // in
    Like,  // like
    Is,    // is
    IsNot, // is not
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Eq => "=".to_string(),
            Operator::Neq => "<>".to_string(),
            Operator::Gt => ">".to_string(),
            Operator::Ge => ">=".to_string(),
            Operator::Lt => "<".to_string(),
            Operator::Le => "<=".to_string(),
            Operator::In => "in".to_string(),
            Operator::Like => "like".to_string(),
            Operator::Is => "is".to_string(),
            Operator::IsNot => "is not".to_string(),
        }
    }
}

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

#[cfg(feature = "postgres")]
use sqlx::{postgres::PgArguments, Postgres};

impl Condition {
    #[cfg(feature = "postgres")]
    pub fn bind_to_query<'a>(
        &self,
        query: sqlx::query::Query<'a, Postgres, PgArguments>,
    ) -> sqlx::query::Query<'a, Postgres, PgArguments> {
        match self {
            Condition::Condition(p, o) => p.bind_to_query(query),
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

    pub fn sql(&self, param_index: usize) -> (String, usize) {
        match self {
            Condition::Condition(p, o) => {
                let field = p.name.clone();
                let op = o.to_string();
                if *o == Operator::In {
                    // in 操作
                    // vec![0; p.value.get_len()].iter().map(|n| );
                    let mut params = vec![];
                    for n in 0..p.value.len() {
                        params.push(format!("${}", param_index + n));
                    }

                    return (
                        format!("{field} {op} ({})", params.join(",")),
                        param_index + p.value.len(),
                    );
                }
                (format!("{field} {op} ${param_index}"), param_index + 1)
            }
            Condition::And(left, right) => {
                let (left_cond, index) = left.sql(param_index);
                let (right_cond, index) = right.sql(index);
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
                let (left_cond, index) = left.sql(param_index);
                let (right_cond, index) = right.sql(index);
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

// pub trait And<T> {
//     fn and(self, cond: T) -> Self;
// }

#[derive(Default)]
pub struct Where {
    cond: Option<Box<Condition>>,
    // params: Vec<Value>,
}

// impl And<(Pair, Operator)> for Where {
//     fn and(mut self, (p, op): (Pair, Operator)) -> Self {
//         if let Some(cond) = self.cond {
//             self.cond = Some(Box::new(Condition::And(
//                 cond,
//                 Box::new(Condition::Condition(p, op)),
//             )));
//         } else {
//             self.cond = Some(Box::new(Condition::Condition(p, op)));
//         }
//         self
//     }
// }

impl Where {
    pub fn new() -> Self {
        Self {
            // params: vec![],
            ..Default::default()
        }
    }

    pub fn and_is(self, p: Pair) -> Self {
        self.and_operator(p, Operator::Is)
    }
    pub fn and_is_not(self, p: Pair) -> Self {
        self.and_operator(p, Operator::IsNot)
    }
    pub fn and_eq(self, p: Pair) -> Self {
        self.and_operator(p, Operator::Eq)
    }
    pub fn and_neq(self, p: Pair) -> Self {
        self.and_operator(p, Operator::Neq)
    }
    pub fn and_lt(self, p: Pair) -> Self {
        self.and_operator(p, Operator::Lt)
    }
    pub fn and_le(self, p: Pair) -> Self {
        self.and_operator(p, Operator::Le)
    }
    pub fn and_gt(self, p: Pair) -> Self {
        self.and_operator(p, Operator::Gt)
    }
    pub fn and_ge(self, p: Pair) -> Self {
        self.and_operator(p, Operator::Ge)
    }
    pub fn and_in(self, p: Pair) -> Self {
        self.and_operator(p, Operator::In)
    }
    pub fn and_like(self, p: Pair) -> Self {
        self.and_operator(p, Operator::Like)
    }

    pub fn and(mut self, w: Where) -> Self {
        if let Some(right_cond) = w.cond {
            if let Some(cond) = self.cond {
                self.cond = Some(Box::new(Condition::And(cond, right_cond)));
            } else {
                self.cond = Some(right_cond);
            }
        }
        self
    }

    pub fn and_operator(mut self, p: Pair, op: Operator) -> Self {
        // self.params.push(p.value.to_owned());
        if let Some(cond) = self.cond {
            self.cond = Some(Box::new(Condition::And(
                cond,
                Box::new(Condition::Condition(p, op)),
            )));
        } else {
            self.cond = Some(Box::new(Condition::Condition(p, op)));
        }

        self
    }

    pub fn or_is(self, p: Pair) -> Self {
        self.or_operator(p, Operator::Is)
    }
    pub fn or_is_not(self, p: Pair) -> Self {
        self.or_operator(p, Operator::IsNot)
    }
    pub fn or_eq(self, p: Pair) -> Self {
        self.or_operator(p, Operator::Eq)
    }
    pub fn or_neq(self, p: Pair) -> Self {
        self.or_operator(p, Operator::Neq)
    }
    pub fn or_lt(self, p: Pair) -> Self {
        self.or_operator(p, Operator::Lt)
    }
    pub fn or_le(self, p: Pair) -> Self {
        self.or_operator(p, Operator::Le)
    }
    pub fn or_gt(self, p: Pair) -> Self {
        self.or_operator(p, Operator::Gt)
    }
    pub fn or_ge(self, p: Pair) -> Self {
        self.or_operator(p, Operator::Ge)
    }
    pub fn or_in(self, p: Pair) -> Self {
        self.or_operator(p, Operator::In)
    }
    pub fn or_like(self, p: Pair) -> Self {
        self.or_operator(p, Operator::Like)
    }

    pub fn or(mut self, w: Where) -> Self {
        if let Some(right_cond) = w.cond {
            if let Some(cond) = self.cond {
                self.cond = Some(Box::new(Condition::Or(cond, right_cond)));
            } else {
                self.cond = Some(right_cond);
            }
        }
        self
    }

    pub fn or_operator(mut self, p: Pair, op: Operator) -> Self {
        if let Some(cond) = self.cond {
            self.cond = Some(Box::new(Condition::Or(
                cond,
                Box::new(Condition::Condition(p, op)),
            )));
        } else {
            self.cond = Some(Box::new(Condition::Condition(p, op)));
        }
        self
    }

    pub fn sql(&self, param_index: usize) -> (String, usize) {
        if let Some(cond) = &self.cond {
            cond.sql(param_index)
        } else {
            ("".to_string(), param_index)
        }
    }

    // pub fn and(mut self, c: Condition) -> Self {
    //     self.cond = Box::new(Condition::And(self.cond, Box::new(c)));
    //     self
    // }

    // pub fn or(mut self, c: Condition) -> Self {
    //     self.cond = Box::new(Condition::Or(self.cond, Box::new(c)));
    //     self
    // }
}
