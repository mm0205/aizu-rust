//! ITP1_4_Bの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_4_B](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_4_B)

use std::f64::consts::PI;
use std::io::stdin;

/// ITP1_4_Bの回答(エントリポイント)。
#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Err(_) = stdin().read_line(&mut line) {
            return;
        }

        let r = match line.trim().parse::<f64>() {
            Ok(n) => n,
            _ => return
        };
        let result = compute_result(r);
        println!("{} {}", result.area, result.circumference);
    }
}

#[derive(Debug, PartialEq)]
struct ResultSet {
    area: f64,
    circumference: f64,
}

fn compute_result(r: f64) -> ResultSet {
    let area = PI * r.powi(2);
    let circumference = 2.0 * PI * r;
    ResultSet { area, circumference }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_float_eq {
        ($expected:expr, $actual:expr, $error:expr) => {
            assert!(($expected - $actual).abs() < $error)
        }
    }

    #[test]
    fn test_compute_result() {
        let result = compute_result(2.0);
        assert_float_eq!(12.566371, result.area, 10_f64.powi(-5));
        assert_float_eq!(12.566371, result.circumference, 10_f64.powi(-5));
    }
}