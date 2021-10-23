//! ITP1_4_Aの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_4_A](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_4_A)

use std::io;
use std::str::FromStr;

/// ITP1_4_Aの回答(エントリポイント)。
#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Err(_) = io::stdin().read_line(&mut line) {
            return;
        }

        if let Ok(dataset) = Dataset::from_str(&line) {
            let result = compute_result(&dataset);
            println!("{} {} {}", result.integer_quotient, result.remainder, result.real_quotient);
            continue;
        }

        return;
    }
}

/// ITP1_4_Aのデータセット。
#[derive(Debug, PartialEq)]
struct Dataset {
    a: i32,
    b: i32,
}

impl Dataset {
    fn new(a: i32, b: i32) -> Dataset {
        Dataset { a, b }
    }
}

/// データセットのパースエラー。
#[derive(Debug, PartialEq, Clone, Copy)]
enum DatasetErrorKind {
    /// フィールド数が2では無い。
    NumberOfFields,

    /// N番目のフィールドが整数値ではない。
    NthFieldFormat { n: i32 },
}

/// エラー型
#[derive(Debug, PartialEq, Clone, Copy)]
struct ParseDatasetError {
    /// エラー種別。
    kind: DatasetErrorKind,
}

impl FromStr for Dataset {
    type Err = ParseDatasetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s.trim().split(' ').collect::<Vec<&str>>();
        if fields.len() != 2 {
            return Err(ParseDatasetError { kind: DatasetErrorKind::NumberOfFields });
        }

        let mut result = vec![Ok(0); 2];
        for (n, parsed) in fields.into_iter().map(|x| x.parse::<i32>()).enumerate() {
            result[n] = match parsed {
                Ok(v) => Ok(v),
                Err(_) => Err(ParseDatasetError { kind: DatasetErrorKind::NthFieldFormat { n: (n as i32) } }),
            };
        }

        Ok(Dataset::new(result[0]?, result[1]?))
    }
}

#[derive(Debug, PartialEq)]
struct ResultSet {
    integer_quotient: i32,
    remainder: i32,
    real_quotient: f64,
}

fn compute_result(dataset: &Dataset) -> ResultSet {
    let real_quotient = dataset.a as f64 / dataset.b as f64;
    let integer_quotient = real_quotient.floor() as i32;
    let remainder = dataset.a - dataset.b * integer_quotient;


    ResultSet { integer_quotient, remainder, real_quotient }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dataset_from_text() {
        assert_eq!(Err(ParseDatasetError { kind: DatasetErrorKind::NumberOfFields }), Dataset::from_str(""));
        assert_eq!(Err(ParseDatasetError { kind: DatasetErrorKind::NumberOfFields }), Dataset::from_str("1"));
        assert_eq!(Err(ParseDatasetError { kind: DatasetErrorKind::NumberOfFields }), Dataset::from_str("1 2 3"));
        assert_eq!(Err(ParseDatasetError { kind: DatasetErrorKind::NumberOfFields }), Dataset::from_str("1  2"));

        assert_eq!(Err(ParseDatasetError { kind: DatasetErrorKind::NthFieldFormat { n: 0 } }), Dataset::from_str("a b"));
        assert_eq!(Err(ParseDatasetError { kind: DatasetErrorKind::NthFieldFormat { n: 1 } }), Dataset::from_str("1 b"));

        assert_eq!(Ok(Dataset { a: 1, b: 1 }), Dataset::from_str("1 1"));
        assert_eq!(Ok(Dataset { a: 1000000000, b: 1 }), Dataset::from_str("1000000000 1"));
        assert_eq!(Ok(Dataset { a: 58, b: 39 }), Dataset::from_str("58 39\r\n"));
    }

    macro_rules! assert_float_eq {
        ($expected:expr, $actual:expr, $error:expr) => {
            assert!(($expected - $actual).abs() < $error)
        }
    }

    macro_rules! assert_result_eq {
        ($expected_integer_quotient:expr, $expected_remainder:expr, $expected_real_quotient:expr, $result:expr) => {
            let result = $result;
            assert_eq!($expected_integer_quotient, result.integer_quotient);
            assert_eq!($expected_remainder, result.remainder);
            // 許容誤差 = 10^{-5}
            assert_float_eq!($expected_real_quotient, result.real_quotient, 10_f64.powi(-5));
        }
    }

    #[test]
    fn test_compute_result() {
        assert_eq!(ResultSet { integer_quotient: 1, remainder: 0, real_quotient: 1.0 },
                   compute_result(&Dataset::from_str("1 1").unwrap()));

        assert_result_eq!(1, 1, 1.5, compute_result(&Dataset::from_str("3 2").unwrap()));
        assert_result_eq!(2, 1, 2.5, compute_result(&Dataset::from_str("5 2").unwrap()));
        assert_result_eq!(1, 19, 1.48717949, compute_result(&Dataset::from_str("58 39").unwrap()));
    }
}