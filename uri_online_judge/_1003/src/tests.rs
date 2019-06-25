#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {
        assert_eq!(add(10, 9), 19);
    }

    #[test]
    fn test_two() {
        assert_eq!(add(-10, 4), -6);
    }

    #[test]
    fn test_three() {
        assert_eq!(add(15, -7), 8);
    }
}