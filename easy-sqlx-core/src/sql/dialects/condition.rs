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
    pub fn to_sql(&self, param_index: usize) -> (String, usize) {
        match self {
            Condition::Condition(p, o) => {
                let field = p.name.clone();
                let op = o.to_string();
                if *o == Operator::In {
                    // in 操作
                    // vec![0; p.value.get_len()].iter().map(|n| );
                    let mut params = vec![];
                    for n in 0..p.value.get_len() {
                        params.push(format!("${}", param_index + n));
                    }

                    return (format!("{field} {op} ({})", params.join(",")), param_index + p.value.get_len())
                }
                (format!("{field} {op} ${param_index}"), param_index + 1)
            }
            Condition::And(left, right) => {
                let (left_cond, index) = left.to_sql(param_index);
                let (right_cond, index) = right.to_sql(index);
                (format!("({left_cond}) and ({right_cond})"), index)
            }
            Condition::Or(left, right) => {
                let (left_cond, index) = left.to_sql(param_index);
                let (right_cond, index) = right.to_sql(index);
                (format!("({left_cond}) or ({right_cond})"), index)
            }
        }
    }
}

pub struct Where {
    cond: Condition,
}
