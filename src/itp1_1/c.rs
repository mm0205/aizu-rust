//! ITP1_1_Cの回答。
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_C](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_C)

use std::io;
use std::num::ParseIntError;

/// ITP1_1_Cの回答 エントリポイント。
#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Ok(_) = io::stdin().read_line(&mut line) {
            if let Some(dataset) = line_to_dataset(&line) {
                let result = compute_result(&dataset);
                println!("{} {}", result.area, result.perimeter);
            } else {
                return;
            }
        } else {
            return;
        }
    }
}

/// ITP1_1_Cのデータセット。
#[derive(Debug, PartialEq)]
pub struct Dataset {
    /// 矩形の高さ。
    pub a: i32,
    /// 矩形の幅。
    pub b: i32,
}

impl Dataset {
    /// `Dataset`のインスタンスを作成する。
    ///
    /// # Arguments
    ///
    /// * `a` - 矩形の幅。
    /// * `b` - 矩形の高さ。
    pub fn new(a: i32, b: i32) -> Dataset {
        Dataset { a, b }
    }
}

/// ITP1_1_Cの結果。
#[derive(Debug, PartialEq)]
pub struct ResultSet {
    /// 矩形の面積。
    area: i32,

    /// 矩形の周長。
    perimeter: i32,
}

impl ResultSet {
    /// 新しい`ResultSet`のインスタンスを作成する。
    ///
    /// # Arguments
    ///
    /// * `area` - 矩形の面積。
    /// * `perimeter` - 矩形の周長。
    pub fn new(area: i32, perimeter: i32) -> ResultSet {
        ResultSet { area, perimeter }
    }
}

/// 入力データ1行をデータセットに変換する。
///
/// # Arguments
///
/// * `line` 入力データ1行。 `"2 9"`のように数値 + スペース + 数値 を期待する。
///
fn line_to_dataset(line: &str) -> Option<Dataset> {
    let numbers: Vec<Result<i32, ParseIntError>> = line.trim().split(' ').map(|x| x.parse::<i32>()).collect();
    match numbers[..] {
        [Ok(a), Ok(b)] => Some(Dataset::new(a, b)),
        _ => None
    }
}

fn compute_result(dataset: &Dataset) -> ResultSet {
    let area = dataset.a * dataset.b;
    let perimeter = dataset.a * 2 + dataset.b * 2;
    return ResultSet::new(area, perimeter);
}


#[cfg(test)]
mod test {
    use crate::itp1_1::c::{compute_result, Dataset, line_to_dataset, ResultSet};

    #[test]
    pub fn test_line_to_dataset() {
        assert_eq!(line_to_dataset(""), None);
        assert_eq!(line_to_dataset("a"), None);
        assert_eq!(line_to_dataset("10 a"), None);
        assert_eq!(line_to_dataset("3 8"), Some(Dataset { a: 3, b: 8 }));
    }

    #[test]
    pub fn test_compute_result() {
        assert_eq!(compute_result(&Dataset::new(2, 5)), ResultSet::new(10, 14));
        assert_eq!(compute_result(&Dataset::new(3, 5)), ResultSet::new(15, 16));
    }
}