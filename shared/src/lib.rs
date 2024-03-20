pub mod error;
pub mod game_status;
pub mod response;

pub struct RequestOptions {
    pub limit: i32,
    pub offset: i32,
}

impl RequestOptions {
    pub fn new(limit: Option<i32>, page: Option<i32>) -> RequestOptions {
        let lim = match limit {
            Some(limit) => limit,
            None => 25,
        };
        let offset = match page {
            Some(page) => (page - 1) * lim,
            None => 0,
        };
        RequestOptions { limit: lim, offset }
    }
}
