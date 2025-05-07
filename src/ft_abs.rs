pub fn ft_abs(value: f64) -> f64 {
    if value.is_nan() || value.is_infinite() {
        return value;
    }
    if value < 0.0 {
        -value
    } else {
        value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_number() {
        let result = ft_abs(3.0);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_negative_number() {
        let result = ft_abs(-3.0);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_zero() {
        let result = ft_abs(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_large_negative_number() {
        let result = ft_abs(-1000.0);
        assert_eq!(result, 1000.0);
    }

    #[test]
    fn test_large_positive_number() {
        let result = ft_abs(1000.0);
        assert_eq!(result, 1000.0);
    }

    #[test]
    fn test_small_negative_number() {
        let result = ft_abs(-0.0001);
        assert_eq!(result, 0.0001);
    }

    #[test]
    fn test_small_positive_number() {
        let result = ft_abs(0.0001);
        assert_eq!(result, 0.0001);
    }
}
