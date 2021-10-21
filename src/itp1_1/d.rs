//! ITP1_1_Dの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_D](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_D)

use std::io;

/// ITP1_1_Dの回答(エントリポイント)。
#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();

        if let Ok(_) = io::stdin().read_line(&mut line) {
            if let Some(dataset) = line_to_dataset(&line) {
                let result = compute_hms(&dataset);
                println!("{}:{}:{}", result.hours, result.minutes, result.seconds);
            } else {
                return;
            }
        } else {
            return;
        }
    }
}

/// ITP1_1_Dのデータセット。
#[derive(Debug, PartialEq)]
pub struct Dataset {
    /// 秒。
    seconds: i32,
}

/// ITP1_1_Dの結果。
#[derive(Debug, PartialEq)]
pub struct ResultSet {
    /// 時。
    hours: i32,
    /// 分。
    minutes: i32,
    /// 秒。
    seconds: i32,
}

/// 入力データ1行をデータセットに変換する。
fn line_to_dataset(line: &str) -> Option<Dataset> {
    match line.trim().parse::<i32>() {
        Ok(seconds) => Some(Dataset { seconds }),
        _ => None
    }
}

fn compute_hms(dataset: &Dataset) -> ResultSet {
    let hours = dataset.seconds / (60 * 60);
    let remaining_seconds = dataset.seconds - hours * (60 * 60);
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds - minutes * 60;
    ResultSet { hours, minutes, seconds }
}


#[cfg(test)]
mod test {
    use crate::itp1_1::d::{compute_hms, Dataset, line_to_dataset, ResultSet};

    #[test]
    pub fn test_line_to_dataset() {
        assert_eq!(line_to_dataset("10"), Some(Dataset { seconds: 10 }));
        assert_eq!(line_to_dataset("86400"), Some(Dataset { seconds: 86400 }));
        assert_eq!(line_to_dataset("0"), Some(Dataset { seconds: 0 }));
        assert_eq!(line_to_dataset(""), None);
        assert_eq!(line_to_dataset("a"), None);
    }

    #[test]
    pub fn test_compute_hms() {
        assert_eq!(compute_hms(&Dataset { seconds: 46979 }), ResultSet { hours: 13, minutes: 2, seconds: 59 });
        assert_eq!(compute_hms(&Dataset { seconds: 0 }), ResultSet { hours: 0, minutes: 0, seconds: 0 });
        assert_eq!(compute_hms(&Dataset { seconds: 86399 }), ResultSet { hours: 23, minutes: 59, seconds: 59 });
        assert_eq!(compute_hms(&Dataset { seconds: 86400 }), ResultSet { hours: 24, minutes: 0, seconds: 0 });
    }
}