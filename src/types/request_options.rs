use super::Order;

pub struct RequestOptions<T> {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub order: Option<Order>,
    pub order_field: Option<String>,
    pub search: Option<String>,
    pub filter_field: Option<String>,
    pub filter_value: Option<T>,
}
