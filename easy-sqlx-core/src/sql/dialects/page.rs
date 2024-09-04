
#[derive(Clone, Debug)]
pub enum OrderType {
    Asc,
    Desc,
    None,
}

impl ToString for OrderType {
    fn to_string(&self) -> String {
        match self {
            OrderType::Asc => "asc".to_string(),
            OrderType::Desc => "desc".to_string(),
            OrderType::None => "".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Order {
    pub field: String,
    pub order_type: OrderType,
}

impl ToString for Order {
    fn to_string(&self) -> String {
        format!("{} {}", self.field, self.order_type.to_string())
    }
}

impl Order {
    pub fn asc(field: String) -> Self {
        Self {
            field,
            order_type: OrderType::Asc,
        }
    }

    pub fn desc(field: String) -> Self {
        Self {
            field,
            order_type: OrderType::Desc,
        }
    }

    pub fn new(field: String) -> Self {
        Self {
            field,
            order_type: OrderType::None,
        }
    }
}
