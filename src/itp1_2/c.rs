//! ITP1_2_Cの回答。
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_2_C](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_2_C)
use std::io;

/// ITP1_2_Cの回答(エントリポイント)。
#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Ok(_) = io::stdin().read_line(&mut line) {
            if let Some(dataset) = Dataset::from_text(&line) {
                let result = compute_result(dataset);
                println!("{} {} {}", result[0], result[1], result[2]);
                continue;
            }
        }
        return;
    }
}

/// ITP1_2_Cのデータセット。
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
            let x = match t.parse() {
                Ok(n) => n,
                _ => return None
            };
            numbers.push(x);
        }

        Some(Dataset {
            a: *numbers.get(0)?,
            b: *numbers.get(1)?,
            c: *numbers.get(2)?,
        })
    }
}

fn compute_result(dataset: Dataset) -> Vec<i32> {
    let mut result = vec![dataset.a, dataset.b, dataset.c];
    result.sort();
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dataset_from_text() {
        assert_eq!(Some(Dataset { a: 1, b: 2, c: 3 }), Dataset::from_text("1 2 3"));
        assert_eq!(Some(Dataset { a: 10000, b: 500, c: 10 }), Dataset::from_text("10000 500 10"));
        assert_eq!(None, Dataset::from_text(""));
        assert_eq!(None, Dataset::from_text("1"));
        assert_eq!(None, Dataset::from_text("1 2"));
    }

    #[test]
    fn test_sort() {
        assert_eq!(vec![1, 2, 3], compute_result(Dataset::from_text("1 2 3").unwrap()));
        assert_eq!(vec![1, 2, 3], compute_result(Dataset::from_text("3 2 1").unwrap()));
        assert_eq!(vec![1, 2, 3], compute_result(Dataset::from_text("3 1 2").unwrap()));

        assert_eq!(vec![20, 785, 10000], compute_result(Dataset::from_text("10000 785 20").unwrap()));
        assert_eq!(vec![20, 785, 10000], compute_result(Dataset::from_text("20 785 10000").unwrap()));
        assert_eq!(vec![20, 785, 10000], compute_result(Dataset::from_text("20 10000 785").unwrap()));

        assert_eq!(vec![55, 55, 55], compute_result(Dataset::from_text("55 55 55").unwrap()));
        assert_eq!(vec![10, 123, 123], compute_result(Dataset::from_text("123 10 123").unwrap()));
    }
}
