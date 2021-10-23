//! ITP1_2_Bの回答。
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_2_B](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_2_B)

use std::io;

/// ITP1_2_Bの回答(エントリポイント)。
#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Ok(_) = io::stdin().read_line(&mut line) {
            if let Some(dataset) = Dataset::from_text(&line) {
                let result = is_in_order(&dataset);
                let output = result_to_message(result);
                println!("{}", output);
                continue;
            }
        }
        return;
    }
}

/// ITP1_2_Bのデータセット。
#[derive(Debug, PartialEq)]
struct Dataset {
    a: i32,
    b: i32,
    c: i32,
}

//noinspection DuplicatedCode
impl Dataset {
    /// 文字列からデータセットを作成する。
    ///
    /// # Arguments
    ///
    /// * `text` - 文字列。スペース区切りで数字が三つ並んでいることを想定。 例: ("1 2 3")
    fn from_text(text: &str) -> Option<Dataset> {
        let mut numbers: Vec<i32> = Vec::new();
        for x in text.trim().split(' ') {
            let n = match x.parse() {
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

/// データセットの数字が a < b < c 順に並んでいるか判定する。
///
/// # Arguments
///
/// * `dataset` - データセット。
///
fn is_in_order(dataset: &Dataset) -> bool {
    dataset.a < dataset.b && dataset.b < dataset.c
}

/// 並び順の判定結果を文字列に変換する。
///
/// # Arguments
///
/// * `is_in_order` - データセットが順番通りの場合は`true`。
///
fn result_to_message(is_in_order: bool) -> &'static str {
    match is_in_order {
        true => "Yes",
        false => "No",
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dataset_from_text() {
        assert_eq!(None, Dataset::from_text(""));
        assert_eq!(Some(Dataset { a: 0, b: 1, c: 2 }), Dataset::from_text("0 1 2"));
        assert_eq!(Some(Dataset { a: 100, b: 99, c: 98 }), Dataset::from_text("100 99 98"));
    }

    #[test]
    fn test_dataset_is_in_order() {
        assert_eq!(true, is_in_order(&Dataset { a: 0, b: 1, c: 2 }));
        assert_eq!(true, is_in_order(&Dataset { a: 98, b: 99, c: 100 }));
        assert_eq!(true, is_in_order(&Dataset { a: 0, b: 50, c: 96 }));
        assert_eq!(false, is_in_order(&Dataset { a: 100, b: 99, c: 98 }));
        assert_eq!(false, is_in_order(&Dataset { a: 2, b: 1, c: 0 }));
        assert_eq!(false, is_in_order(&Dataset { a: 100, b: 88, c: 10 }));
        assert_eq!(false, is_in_order(&Dataset { a: 10, b: 10, c: 10 }));
        assert_eq!(false, is_in_order(&Dataset { a: 10, b: 12, c: 12 }));
        assert_eq!(false, is_in_order(&Dataset { a: 13, b: 13, c: 21 }));
    }

    #[test]
    fn test_result_to_message() {
        assert_eq!("Yes", result_to_message(true));
        assert_eq!("No", result_to_message(false));
    }
}
