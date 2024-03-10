mod test;

pub fn calculate_offset(limit: i64, page: i64) -> i64 {
    (page - 1) * limit
}
