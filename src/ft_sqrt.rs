use crate::ft_abs::ft_abs;

pub fn ft_sqrt(number: f64) -> f64 {
    if number < 0.0 {
        return f64::NAN;
    }
    if number == 0.0 {
        return 0.0;
    }

    let mut guess: f64 = number;
    let mut prev_guess: f64 = 0.0;
    let precision: f64 = 1.0e-10;

    while ft_abs(guess - prev_guess) > precision {
        prev_guess = guess;
        guess = (guess + number / guess) / 2.0;
    }

    guess
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_numbers() {
        assert!((ft_sqrt(4.0) - 2.0).abs() < 1e-10);
        assert!((ft_sqrt(16.0) - 4.0).abs() < 1e-10);
        assert!((ft_sqrt(25.0) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_zero() {
        assert!((ft_sqrt(0.0) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_negative_number() {
        assert!(ft_sqrt(-1.0).is_nan());
        assert!(ft_sqrt(-100.0).is_nan());
    }

    #[test]
    fn test_large_numbers() {
        let large_number = 1e12;
        assert!((ft_sqrt(large_number) - large_number.sqrt()).abs() < 1e-5);
    }

    #[test]
    fn test_small_numbers() {
        let small_number = 1e-6;
        assert!((ft_sqrt(small_number) - small_number.sqrt()).abs() < 1e-6);
    }

    #[test]
    fn test_high_precision() {
        let precision_case = 2.0;
        assert!((ft_sqrt(precision_case) - precision_case.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_large_float() {
        let large_float = 1e20;
        assert!((ft_sqrt(large_float) - large_float.sqrt()).abs() < 1e-5);
    }

    #[test]
    fn test_tiny_float() {
        let tiny_float: f64 = 1e-10;
        assert!((ft_sqrt(tiny_float) - tiny_float.sqrt()).abs() < 1e-5);
    }

    #[test]
    fn test_perfect_squares() {
        assert_eq!(ft_sqrt(0.0), 0.0);
        assert_eq!(ft_sqrt(1.0), 1.0);
        assert_eq!(ft_sqrt(4.0), 2.0);
        assert_eq!(ft_sqrt(9.0), 3.0);
        assert_eq!(ft_sqrt(16.0), 4.0);
    }

    #[test]
    fn test_non_perfect_squares() {
        assert!((ft_sqrt(2.0) - 1.4142135623730951).abs() < 1e-10);
        assert!((ft_sqrt(3.0) - 1.7320508075688772).abs() < 1e-10);
        assert!((ft_sqrt(5.0) - 2.23606797749979).abs() < 1e-10);
    }

    #[test]
    fn test_fractions() {
        assert!((ft_sqrt(0.25) - 0.5).abs() < 1e-10);
        assert!((ft_sqrt(0.5) - 0.7071067811865476).abs() < 1e-10);
        assert!((ft_sqrt(2.25) - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_negative() {
        assert!(ft_sqrt(-1.0).is_nan());
        assert!(ft_sqrt(-0.5).is_nan());
        assert!(ft_sqrt(-f64::MIN_POSITIVE).is_nan());
    }

    #[test]
    fn test_large_numbers2() {
        assert!((ft_sqrt(1e16) - 1e8).abs() < 1e-10);
        assert!((ft_sqrt(1e20) - 1e10).abs() < 1e-10);
    }
}
