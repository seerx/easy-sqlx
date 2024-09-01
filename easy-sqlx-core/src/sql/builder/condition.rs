use super::pair::Pair;


pub enum Operator {
    Eq, // =
    Neq, // <>
    Gt, // >
    Ge, // >=
    Lt, // <
    Le, // <=
    In, // in
    Like, // like
    Is, // is
    IsNot, // is not
}

pub struct Condition {
    // pub andor: AndOr,
    pub data: Pair,
    pub operator: Operator,
    
}

pub enum AndOr {
    And(Condition, Condition),
    And1(Box<AndOr>, Condition),
    And2(Condition, Box<AndOr>),
    And3(Box<AndOr>, Box<AndOr>),
    Or(Condition, Condition),
}

pub struct Where {}