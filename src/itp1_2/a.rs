//! ITP1_2_Aの回答。
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_2_A](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_2_A)

use std::cmp::Ordering;
use std::io;

/// ITP1_2_Aの回答(エントリポイント)。
#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Ok(_) = io::stdin().read_line(&mut line) {
            if let Some(dataset) = Dataset::from_text(&line) {
                let output = compute_message(&dataset);
                println!("{}", output);
            } else {
                return;
            }
        } else {
            return;
        }
    }
}

/// ITP1_2_Aのデータセット。
#[derive(Debug, PartialEq)]
struct Dataset {
    /// 比較対象(LHS)。
    a: i32,
    /// 比較対象(RHS)。
    b: i32,
}

impl Dataset {
    fn from_text(line: &str) -> Option<Dataset> {
        let numbers: Vec<Option<i32>> = line.trim()
            .split(' ')
            .map(|x| match x.parse::<i32>() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect();

        let a = *numbers.get(0)?;
        let b = *numbers.get(1)?;
        Some(Dataset {
            a: a?,
            b: b?,
        })
    }

    fn order_of_a_b(&self) -> Ordering {
        self.a.cmp(&self.b)
    }
}

fn compute_message(dataset: &Dataset) -> &str {
    match dataset.order_of_a_b() {
        Ordering::Less => "a < b",
        Ordering::Equal => "a == b",
        Ordering::Greater => "a > b"
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_dataset_from_text() {
        assert_eq!(None, Dataset::from_text(""));
        assert_eq!(None, Dataset::from_text("1"));
        assert_eq!(None, Dataset::from_text("1 a"));
        assert_eq!(Some(Dataset { a: 1, b: 1 }), Dataset::from_text("1 1"));
        assert_eq!(Some(Dataset { a: -999, b: -1000 }), Dataset::from_text("-999 -1000"));
        assert_eq!(Some(Dataset { a: 999, b: 1000 }), Dataset::from_text("999 1000"));
    }

    #[test]
    pub fn test_order_of_a_b() {
        assert_eq!(Ordering::Equal, Dataset::from_text("1 1").unwrap().order_of_a_b());
        assert_eq!(Ordering::Greater, Dataset::from_text("-999 -1000").unwrap().order_of_a_b());
        assert_eq!(Ordering::Less, Dataset::from_text("999 1000").unwrap().order_of_a_b());
    }

    #[test]
    pub fn test_message() {
        assert_eq!("a == b", compute_message(&Dataset::from_text("1 1").unwrap()));
        assert_eq!("a > b", compute_message(&Dataset::from_text("-999 -1000").unwrap()));
        assert_eq!("a < b", compute_message(&Dataset::from_text("999 1000").unwrap()));
    }
}