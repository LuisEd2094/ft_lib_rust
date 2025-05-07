pub fn ft_powi(number: f64, power: i32) -> f64 {
    let mut result: f64 = 1.0;
    let mut n: f64 = number;
    let mut p: i32 = power;

    if p < 0 {
        n = 1.0 / n;
        p = -p;
    }

    while p > 0 {
        if p % 2 == 1 {
            result *= n;
        }
        n *= n;
        p /= 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_power() {
        assert!((ft_powi(2.0, 3) - 8.0).abs() < 1e-10);
        assert!((ft_powi(5.0, 0) - 1.0).abs() < 1e-10);
        assert!((ft_powi(3.0, 4) - 81.0).abs() < 1e-10);
    }

    #[test]
    fn test_negative_power() {
        assert!((ft_powi(2.0, -3) - 0.125).abs() < 1e-10);
        assert!((ft_powi(5.0, -2) - 0.04).abs() < 1e-10);
        assert!((ft_powi(10.0, -1) - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_zero_and_one_base() {
        assert!((ft_powi(0.0, 5) - 0.0).abs() < 1e-10);
        assert!((ft_powi(1.0, 1000) - 1.0).abs() < 1e-10);
        assert!((ft_powi(0.0, 0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_negative_base() {
        assert!((ft_powi(-2.0, 3) - (-8.0)).abs() < 1e-10);
        assert!((ft_powi(-2.0, 4) - 16.0).abs() < 1e-10);
    }

    #[test]
    fn test_positive_exponent() {
        assert_eq!(ft_powi(2.0, 3), 8.0);
        assert_eq!(ft_powi(5.0, 4), 625.0);
        assert_eq!(ft_powi(1.5, 3), 3.375);
        assert_eq!(ft_powi(10.0, 0), 1.0);
        assert_eq!(ft_powi(0.0, 5), 0.0);
    }

    #[test]
    fn test_negative_exponent() {
        assert_eq!(ft_powi(2.0, -3), 0.125);
        assert!((ft_powi(5.0, -2) - 0.04).abs() < 1e-10);
        assert_eq!(ft_powi(1.5, -3), 0.2962962962962963);
        assert_eq!(ft_powi(10.0, -0), 1.0);
    }

    #[test]
    fn test_zero_base() {
        assert_eq!(ft_powi(0.0, 5), 0.0);
        assert_eq!(ft_powi(0.0, 0), 1.0);
        assert_eq!(ft_powi(0.0, -3), f64::INFINITY);
    }

    #[test]
    fn test_one_base() {
        assert_eq!(ft_powi(1.0, 100), 1.0);
        assert_eq!(ft_powi(1.0, -100), 1.0);
        assert_eq!(ft_powi(1.0, 0), 1.0);
    }

    #[test]
    fn test_large_exponents() {
        assert_eq!(ft_powi(2.0, 30), 1073741824.0);
        assert_eq!(ft_powi(2.0, -30), 9.313225746154785e-10);
        assert!((ft_powi(1.0001, 10000) - 2.7181459268249255).abs() < 1e-10);
    }

    #[test]
    fn test_fractional_base() {
        assert_eq!(ft_powi(0.5, 2), 0.25);
        assert_eq!(ft_powi(0.5, -2), 4.0);
        assert_eq!(ft_powi(2.5, 3), 15.625);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(ft_powi(f64::MAX, 1), f64::MAX);
        assert_eq!(ft_powi(f64::MIN_POSITIVE, 1), f64::MIN_POSITIVE);
        assert_eq!(ft_powi(f64::INFINITY, 2), f64::INFINITY);
        assert_eq!(ft_powi(f64::NEG_INFINITY, 3), f64::NEG_INFINITY);
        assert!(ft_powi(f64::NAN, 5).is_nan());
    }
}
