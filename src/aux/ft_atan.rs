pub fn ft_atan(t: f64) -> f64 {
    const P0: f64 = -8.7506086000319042e-01;
    const P1: f64 = -1.6157537187333657e+01;
    const P2: f64 = -7.5008557923147045e+01;
    const P3: f64 = -5.3412667097952299e+01;
    const Q0: f64 = 2.4858464901423064e+01;
    const Q1: f64 = 3.6503008330082296e+02;
    const Q2: f64 = 1.2752747779636169e+03;

    let t_squared = t * t;
    let numerator = t * (P0 + t_squared * (P1 + t_squared * (P2 + t_squared * P3)));
    let denominator = Q0 + t_squared * (Q1 + t_squared * (Q2 + t_squared));
    numerator / denominator
}