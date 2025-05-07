use std::f64::consts::PI;
use crate::ft_abs::ft_abs;

pub fn ft_cos(x: f64) -> f64 {
    if x == 0.0 {
        return 1.0;
    }
    let x_reduced: f64 = x % (2.0 * PI);
    let x_abs: f64 = ft_abs(x_reduced);

    let (x_normalized, sign) = if x_abs <= PI / 2.0 {
        (x_abs, 1.0)
    } else if x_abs <= PI {
        (PI - x_abs, -1.0)
    } else if x_abs <= 3.0 * PI / 2.0 {
        (x_abs - PI, -1.0)
    } else {
        (2.0 * PI - x_abs, 1.0)
    };

    // Polynomial approximation for cos(x) in [0, π/2]
    sign * compute_cos_poly(x_normalized)
}

/// Compute cos(x) for x in [0, π/2] using a Chebyshev polynomial.
fn compute_cos_poly(x: f64) -> f64 {
    const C0: f64 = 0.9999999999999999934776;
    const C1: f64 = -0.4999999999999999993526;
    const C2: f64 = 0.04166666666666666564023;
    const C3: f64 = -0.001388888888888888803101;
    const C4: f64 = 0.00002480158730158702330044;
    const C5: f64 = -0.0000002755731922393322564216;

    let x_squared: f64 = x * x;
    C0 + x_squared * (C1 + x_squared * (C2 + x_squared * (C3 + x_squared * (C4 + x_squared * C5))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    const RELAXED_EPSILON: f64 = 1e-5;

    fn approx_eq_relaxed(a: f64, b: f64) -> bool {
        (a - b).abs() < RELAXED_EPSILON
    }

    #[test]
    fn test_cos_known_angles() {
        // Test exact known values
        assert!(approx_eq(ft_cos(0.0), 1.0));
        assert!(ft_cos(PI / 2.0).abs() < EPSILON);
        assert!(approx_eq(ft_cos(PI), -1.0));
        assert!(ft_cos(3.0 * PI / 2.0).abs() < EPSILON);
        assert!(approx_eq(ft_cos(2.0 * PI), 1.0));
    }

    #[test]
    fn test_cos_symmetry() {
        // cos(x) = cos(-x)
        assert!(approx_eq(ft_cos(PI / 3.0), ft_cos(-PI / 3.0)));
        assert!(approx_eq(ft_cos(2.0), ft_cos(-2.0)));
    }

    #[test]
    fn test_cos_shift_by_2pi() {
        // cos(x) = cos(x + 2π)
        assert!(approx_eq(ft_cos(PI / 4.0), ft_cos(PI / 4.0 + 2.0 * PI)));
        assert!(approx_eq(ft_cos(PI / 6.0), ft_cos(PI / 6.0 - 2.0 * PI)));
    }

    #[test]
    fn test_cos_random_values() {
        // Some random values compared to std lib cosine
        assert!(approx_eq(ft_cos(0.5), 0.5f64.cos()));
        assert!(approx_eq(ft_cos(1.0), 1.0f64.cos()));
        assert!(approx_eq(ft_cos(2.5), 2.5f64.cos()));
        assert!(approx_eq_relaxed(ft_cos(5.0), 5.0f64.cos()));
    }

    #[test]
    fn test_basic_angles() {
        assert_eq!(ft_cos(0.0), 1.0);
        assert!((ft_cos(PI / 2.0) - 0.0).abs() < EPSILON);
        assert!((ft_cos(PI) + 1.0).abs() < EPSILON);
        assert!((ft_cos(3.0 * PI / 2.0) - 0.0).abs() < EPSILON);
        assert!((ft_cos(2.0 * PI) - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_quadrants() {
        assert!((ft_cos(PI / 4.0) - 0.7071067811865476).abs() < EPSILON);
        assert!((ft_cos(3.0 * PI / 4.0) + 0.7071067811865475).abs() < EPSILON);
        assert!((ft_cos(5.0 * PI / 4.0) + 0.7071067811865477).abs() < EPSILON);
        assert!((ft_cos(7.0 * PI / 4.0) - 0.7071067811865477).abs() < EPSILON);
    }

    #[test]
    fn test_periodicity() {
        const EPSILON: f64 = 1e-6;
        assert!((ft_cos(2.0 * PI) - ft_cos(4.0 * PI)).abs() < EPSILON);
        assert!((ft_cos(PI / 3.0) - ft_cos(PI / 3.0 + 2.0 * PI)).abs() < EPSILON);
        assert!((ft_cos(1.0) - ft_cos(1.0 + 2.0 * PI)).abs() < EPSILON);
    }
    

    #[test]
    fn test_negative_angles() {
        assert_eq!(ft_cos(1.0), ft_cos(-1.0));
        assert_eq!(ft_cos(PI / 3.0), ft_cos(-PI / 3.0));
        assert_eq!(ft_cos(PI), ft_cos(-PI));
    }

    #[test]
    fn test_large_angles() {
        assert!((ft_cos(1000.0 * PI) - 1.0).abs() < EPSILON);
        assert!((ft_cos(1000.0 * PI + PI / 2.0) - 0.0).abs() < EPSILON);
        assert!((ft_cos(1e6) - ft_cos(1e6 % (2.0 * PI))).abs() < EPSILON);
    }

    #[test]
    fn test_precision() {
        assert!((ft_cos(1.0) - 0.5403023058681398).abs() < EPSILON);
        assert!((ft_cos(0.1) - 0.9950041652780258).abs() < EPSILON);
        assert!((ft_cos(0.01) - 0.9999500004166653).abs() < EPSILON);
    }

    #[test]
    fn test_edge_cases() {
        assert!(ft_cos(f64::INFINITY).is_nan());
        assert!(ft_cos(f64::NEG_INFINITY).is_nan());
        assert!(ft_cos(f64::NAN).is_nan());
    }
}
