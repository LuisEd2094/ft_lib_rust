use crate::aux::ft_abs::ft_abs;

pub fn ft_cbrt(n: f64) -> f64 {
    if n == 0.0 {
        return 0.0;
    }

    let mut x: f64 = ft_abs(n);
    let epsilon: f64 = 1e-10;
    let max_iter: i32 = 100;

    for _ in 0..max_iter {
        let next_x: f64 = (2.0 * x + n / (x * x)) / 3.0;
        if ft_abs(next_x - x) < epsilon {
            break;
        }
        x = next_x;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ft_cbrt_positive() {
        assert_eq!(format!("{:.4}", ft_cbrt(27.0)), "3.0000");
        assert_eq!(format!("{:.4}", ft_cbrt(8.0)), "2.0000");
        assert_eq!(format!("{:.4}", ft_cbrt(125.0)), "5.0000");
    }

    #[test]
    fn test_ft_cbrt_negative() {
        assert_eq!(format!("{:.4}", ft_cbrt(-27.0)), "-3.0000");
        assert_eq!(format!("{:.4}", ft_cbrt(-8.0)), "-2.0000");
        assert_eq!(format!("{:.4}", ft_cbrt(-125.0)), "-5.0000");
        assert_eq!(format!("{:.4}", ft_cbrt(-1.0)), "-1.0000");
    }

    #[test]
    fn test_ft_cbrt_zero() {
        assert_eq!(format!("{:.4}", ft_cbrt(0.0)), "0.0000");
    }

    #[test]
    fn test_ft_cbrt_large() {
        assert_eq!(format!("{:.4}", ft_cbrt(1_000_000.0)), "100.0000");
    }

}
