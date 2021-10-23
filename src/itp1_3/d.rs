//! ITP1_3_Dの回答。
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_3_D](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_3_D)

use std::io;

/// ITP1_3_Dの回答(エントリポイント)。
#[allow(dead_code)]
fn main() {
    loop {
        let mut line = String::new();
        if let Err(_) = io::stdin().read_line(&mut line) {
            return;
        }
        if let Some(dataset) = Dataset::from_text(&line) {
            let result = compute_itp1_3_d(&dataset);
            println!("{}", result);
            continue;
        }
        return;
    }
}

#[derive(Debug, PartialEq)]
struct Dataset {
    a: i32,
    b: i32,
    c: i32,
}

//noinspection DuplicatedCode
impl Dataset {
    fn from_text(text: &str) -> Option<Dataset> {
        let mut numbers: Vec<i32> = Vec::new();
        for t in text.trim().split(' ') {
            let n = match t.parse() {
                Ok(n) => n,
                _ => return None
            };
            numbers.push(n);
        }

        Some(Dataset {
            a: *numbers.get(0)?,
            b: *numbers.get(1)?,
            c: *numbers.get(2)?,
        })
    }
}

/// `n`の約数を求める。
fn compute_divisors_of(n: i32) -> Vec<i32> {
    let mut result = Vec::new();

    let mut d = 1;
    let mut dd = d * d;

    while dd <= n {
        if n % d == 0 {
            result.push(d);

            // dがnの約数の場合は `n / d` も n の約数になる。
            // n == d^2の場合は、 `d = n / d` なので2回追加しないようにする。
            // ※ d_i <= sqrt(n) <= n / d_i
            if dd != n {
                result.push(n / d);
            }
        }

        d += 1;
        dd = d * d;
    };
    result.sort();
    result
}

/// `vec`の中の要素v_iのうち、 `min <= v_i <= max` を満たすものだけを抽出した新しい`Vec<i32>`を作成する。
///
/// # Arguments
///
/// * `vec` - 元データ列。
/// * `min` - 新しいVecの最小値。
/// * `max` - 新しいVecの最大値。
fn filter_by_min_max(vec: &Vec<i32>, min: i32, max: i32) -> Vec<i32> {
    vec.iter().filter_map(|x| match min <= *x && *x <= max {
        true => Some(*x),
        false => None
    }).collect()
}

fn compute_itp1_3_d(dataset: &Dataset) -> i32 {
    let divisors = compute_divisors_of(dataset.c);
    let valid_divisors = filter_by_min_max(&divisors, dataset.a, dataset.b);
    valid_divisors.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dataset_from_text() {
        assert_eq!(Some(Dataset { a: 5, b: 14, c: 80 }), Dataset::from_text("5 14 80"));
        assert_eq!(Some(Dataset { a: 1, b: 14, c: 80 }), Dataset::from_text("1 14 80"));
        assert_eq!(Some(Dataset { a: 1, b: 14, c: 10000 }), Dataset::from_text("1 14 10000"));

        assert_eq!(None, Dataset::from_text(""));
        assert_eq!(None, Dataset::from_text("1"));
        assert_eq!(None, Dataset::from_text("1 2"));
        assert_eq!(None, Dataset::from_text("1 2 "));
    }

    #[test]
    fn test_compute_divisors_of() {
        assert_eq!(vec![1], compute_divisors_of(1));
        assert_eq!(vec![1, 2], compute_divisors_of(2));
        assert_eq!(vec![1, 3], compute_divisors_of(3));
        assert_eq!(vec![1, 2, 3, 6], compute_divisors_of(6));
    }

    #[test]
    fn test_filter_by_min_max() {
        assert_eq!(vec![2, 3, 4], filter_by_min_max(&vec![1, 2, 3, 4, 5], 2, 4));
        assert_eq!(vec![1, 2, 3, 4, 5], filter_by_min_max(&vec![1, 2, 3, 4, 5], -1, 10));
        assert_eq!(vec![1], filter_by_min_max(&vec![1, 2, 3, 4, 5], 1, 1));
        assert_eq!(vec![1, 2, 3], filter_by_min_max(&vec![1, 2, 3, 4, 5], 1, 3));
        assert_eq!(vec![5], filter_by_min_max(&vec![1, 2, 3, 4, 5], 5, 5));
        assert_eq!(vec![4, 5], filter_by_min_max(&vec![1, 2, 3, 4, 5], 4, 5));
    }

    #[test]
    fn test_compute() {
        assert_eq!(3, compute_itp1_3_d(&Dataset::from_text("5 14 80").unwrap()));
    }
}