//! ITP1_1_Bの回答。
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_B](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_B)

use std::io;

/// ITP1_1_Bの回答。
#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Ok(_) = io::stdin().read_line(&mut line) {
            let dataset = helpers::line_to_dataset(&line);

            if let Some(ds) = dataset {
                let result = helpers::cube_of(&ds);
                println!("{}", result);
            } else {
                return;
            }
        } else {
            return;
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Dataset {
    num: i32,
}

mod helpers {
    use super::*;

    /// 入力1行をデータセットに変換する。
    ///
    /// # Arguments
    ///
    /// * `line` - 入力データ(1行)。
    ///
    pub fn line_to_dataset(line: &str) -> Option<Dataset> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }
        match line.parse::<i32>() {
            Ok(num) => Some(Dataset { num }),
            Err(_) => None
        }
    }

    /// `dataset.num`の3乗を計算する。
    ///
    /// # Arguments
    ///
    /// * `dataset` 計算対象の整数値を含むデータセット。
    ///
    pub fn cube_of(dataset: &Dataset) -> i32 {
        dataset.num.pow(3)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        pub fn test_line_to_dataset() {
            let actual = line_to_dataset("test");
            assert!(actual.is_none(), "test");

            let actual = line_to_dataset("3");
            assert_eq!(actual, Some(Dataset { num: 3 }));
        }

        #[test]
        pub fn test_cube_of() {
            assert_eq!(1000, cube_of(&Dataset { num: 10 }));
            assert_eq!(1, cube_of(&Dataset { num: 1 }));
            assert_eq!(8, cube_of(&Dataset { num: 2 }));
            assert_eq!(27, cube_of(&Dataset { num: 3 }));
        }
    }
}
