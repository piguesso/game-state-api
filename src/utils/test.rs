#[cfg(test)]
mod utils_test {
    use crate::utils::calculate_offset;

    #[test]
    fn test_calculate_offset() {
        assert_eq!(calculate_offset(10, 1), 0);
        assert_eq!(calculate_offset(10, 2), 10);
        assert_eq!(calculate_offset(10, 3), 20);
    }
}
