//! ITP1_10_Aの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_10_A](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_10_A)

#[allow(dead_code)]
pub fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}", Dataset::from(line.as_str()).distance());
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Dataset {
    p1: Point,
    p2: Point,
}

impl Dataset {
    fn distance(&self) -> f64 {
        ((self.p1.x - self.p2.x).powi(2)
            + (self.p1.y - self.p2.y).powi(2)).sqrt()
    }
}

impl From<&str> for Dataset {
    fn from(s: &str) -> Self {
        let v: Vec<f64> = s.trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Dataset { p1: Point::new(v[0], v[1]), p2: Point::new(v[2], v[3]) }
    }
}

//noinspection DuplicatedCode
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_whitespace() {
        let v: Vec<&str> = "a    b c".split_whitespace().collect();
        assert_eq!(3, v.len());
        assert_eq!("a", v[0]);
        assert_eq!("b", v[1]);
        assert_eq!("c", v[2]);
    }

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
    fn test_dataset_from_str() {
        let dataset = Dataset::from("0 0 1 1");
        assert_eq_f!(0f64, dataset.p1.x);
        assert_eq_f!(0f64, dataset.p1.y);
        assert_eq_f!(1f64, dataset.p2.x);
        assert_eq_f!(1f64, dataset.p2.y);
    }

    #[test]
    fn test_dataset_distance() {
        assert_eq_f!(1.41421356, Dataset::from("0 0 1 1").distance());
    }
}