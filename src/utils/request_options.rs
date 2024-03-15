pub enum OrderEnum {
    Asc,
    Desc,
}

impl OrderEnum {
    pub fn from_string(order: &str) -> Self {
        match order {
            "asc" => OrderEnum::Asc,
            "desc" => OrderEnum::Desc,
            _ => OrderEnum::Asc,
        }
    }

    fn to_string(&self) -> String {
        match self {
            OrderEnum::Asc => "asc".to_string(),
            OrderEnum::Desc => "desc".to_string(),
        }
    }
}

pub struct Order {
    pub field: String,
    pub order: String,
}

impl Order {
    pub fn new(field: String, order: OrderEnum) -> Self {
        Order {
            field,
            order: order.to_string(),
        }
    }

    fn change_order(&mut self, order: OrderEnum) {
        self.order = order.to_string();
    }

    fn change_field(&mut self, field: String) {
        self.field = field;
    }
}

pub enum OperatorEnum {
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
    Like,
    Ilike,
    IsNull,
    In,
    NotIn,
}

pub struct Filter {
    pub field: String,
    pub value: String,
    pub operator: OperatorEnum,
}

impl Filter {
    pub fn new(field: String, value: String, operator: OperatorEnum) -> Self {
        Filter {
            field,
            value,
            operator,
        }
    }
}

pub struct RequestOptions {
    pub limit: i32,
    pub offset: i32,
    pub order: Order,
    pub search: Option<String>,
    pub filters: Option<Vec<Filter>>,
}

impl RequestOptions {
    pub fn new(
        limit: Option<i32>,
        page: Option<i32>,
        order: Option<Order>,
        search: Option<String>,
        filters: Option<Vec<Filter>>,
    ) -> Self {
        let limit = match limit {
            Some(l) => l,
            None => 25,
        };

        let offset = match page {
            Some(p) => {
                if p > 1 {
                    (p - 1) * limit
                } else {
                    0
                }
            }
            None => 0,
        };

        let order = match order {
            Some(o) => o,
            None => Order::new("created_at".to_string(), OrderEnum::Desc),
        };

        RequestOptions {
            limit,
            offset,
            order,
            search,
            filters,
        }
    }
}
