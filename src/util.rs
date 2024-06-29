/// https://en.wikipedia.org/wiki/Bessel_function#Bessel_functions_of_the_first_kind
/// J_a(x) = Sigma\[m->inf]( -1^m / (m!(m+a!)) * (x/2)^2m+a )
pub fn bessel_j(a: i32, x: f32) -> f32 {
    let x2 = x / 2.;

    // m = 0
    let mut term = x2.powi(a) / (1..=a).product::<i32>() as f32;
    let mut sum = term;

    for m in 1..30 {
        term *= -1. * x2 * x2 / m as f32 / (m + a) as f32;
        sum += term;
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bessel() {
        // https://www.statisticshowto.com/wp-content/uploads/2018/09/bessel-tables.pdf
        assert!((bessel_j(0, 0.0) - 1.0000).abs() < 1.0e-4);
        assert!((bessel_j(1, 0.0) - 0.0000).abs() < 1.0e-4);
        assert!((bessel_j(2, 0.0) - 0.0000).abs() < 1.0e-4);
        assert!((bessel_j(0, 2.2) - 0.1104).abs() < 1.0e-4);
        assert!((bessel_j(2, 3.9) - 0.3879).abs() < 1.0e-4);
        assert!((bessel_j(6, 4.4) - 0.0763).abs() < 1.0e-4);
        assert!((bessel_j(9, 4.9) - 0.0047).abs() < 1.0e-4);
    }
}
