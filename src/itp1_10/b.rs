//! ITP1_10_Bの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_10_B&lang=ja](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_10_B&lang=ja)

use std::ops::Deref;

/// ITP1_10_Bの回答
#[allow(dead_code)]
pub fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let v: Vec<i32> = line.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    let dataset = Dataset::new(v[0], v[1], v[2]);
    let answer = dataset.compute_answer();
    println!("{}", answer.s);
    println!("{}", answer.l);
    println!("{}", answer.h);

}

#[derive(Debug, Copy, Clone)]
enum Radian {
    Of(f64),
}

impl Radian {
    fn from_degree(degree: i32) -> Radian {
        Radian::Of((degree as f64).to_radians())
    }
}

impl Deref for Radian {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        let Self::Of(rad) = self;
        rad
    }
}

#[derive(Debug)]
struct Dataset {
    a: f64,
    b: f64,
    c: f64,
    theta: Radian,
}

impl Dataset {
    /// 2辺とその間の角を指定してインスタンスを作成する。
    fn new(a: i32, b: i32, theta: i32) -> Self {
        let a = a as f64;
        let b = b as f64;
        let theta = Radian::from_degree(theta);
        let c = Self::compute_c(a, b, theta);
        Dataset { a, b, c, theta }
    }

    fn compute_c(a: f64, b: f64, theta: Radian) -> f64 {
        let c_2 = a.powi(2) + b.powi(2) - 2f64 * a * b * (*theta).cos();
        c_2.sqrt()
    }

    fn compute_answer(&self) -> Answer {
        let h = self.b * self.theta.sin();
        let l = self.a + self.b + self.c;
        let s = self.a * h / 2f64;

        Answer::new(s, l, h)
    }
}

#[derive(Debug)]
struct Answer {
    s: f64,
    l: f64,
    h: f64,
}

impl Answer {
    fn new(s: f64, l: f64, h: f64) -> Self {
        Self { s, l, h }
    }
}

//noinspection DuplicatedCode
#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_eq_f {
        ($x:expr, $y:expr, $message: expr) => {
            let d = $x - $y;
            assert!(d.abs() < 0.0001, $message);
        };
        ($x:expr, $y:expr) => {
            let d = $x - $y;
            assert!(d.abs() < 0.0001);
        }
    }

    #[test]
    fn test_c() {
        let a = 4f64;
        let b = 3f64;
        let theta = 90f64;

        let theta = theta.to_radians();
        let c_2 = a.powi(2) + b.powi(2) - 2f64 * a * b * theta.cos();
        assert_eq_f!(25f64, c_2);
    }

    #[test]
    fn test_compute_c() {
        assert_eq_f!(5f64, Dataset::compute_c(4f64, 3f64, Radian::Of(90f64.to_radians())));
    }

    #[test]
    fn test_s() {
        assert_eq_f!(6f64, Dataset::new(4, 3, 90).compute_answer().s);
    }

    #[test]
    fn test_l() {
        assert_eq_f!(12f64, Dataset::new(4, 3, 90).compute_answer().l);
    }

    #[test]
    fn test_h() {
        assert_eq_f!(3f64, Dataset::new(4, 3, 90).compute_answer().h);
    }
}