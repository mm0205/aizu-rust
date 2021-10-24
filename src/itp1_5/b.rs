//! ITP1_5_Bの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_5_B](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_5_B)

use std::io::stdin;
use std::str::FromStr;

//noinspection DuplicatedCode
/// ITP1_5_Bの回答
#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Err(_) = stdin().read_line(&mut line) {
            return;
        }

        if let Ok(dataset) = Dataset::from_str(&line) {
            if dataset.is_end() {
                return;
            }
            let result = create_rect(&dataset);
            println!("{}\n", result);
            continue;
        }
        return;
    }
}

#[derive(Debug, PartialEq)]
struct Dataset {
    h: i32,
    w: i32,
}

//noinspection DuplicatedCode
impl Dataset {
    fn new(h: i32, w: i32) -> Dataset {
        Dataset { h, w }
    }

    fn is_end(&self) -> bool {
        self.h == 0 && self.w == 0
    }
}

//noinspection DuplicatedCode
impl FromStr for Dataset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.trim().split(' ').into_iter().map(|x| x.parse::<i32>()).collect::<Vec<_>>();
        if parsed.len() != 2 {
            return Err(());
        }

        let mut result = vec![0; 2];

        for (i, p) in parsed.into_iter().enumerate() {
            result[i] = match p {
                Ok(n) => n,
                _ => return Err(())
            };
        }

        Ok(Dataset::new(result[0], result[1]))
    }
}

fn create_rect(dataset: &Dataset) -> String {
    let mut result = Vec::new();
    let top_and_bottom = "#".repeat(dataset.w as usize);

    let mut others = "#".to_string();
    others.push_str(&".".repeat((dataset.w - 2) as usize));
    others.push_str("#");

    result.push(top_and_bottom.clone());
    for _ in 0..(dataset.h - 2) {
        result.push(others.clone());
    }
    result.push(top_and_bottom);
    result.join("\n")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dataset_new() {
        assert_eq!(Dataset { h: 10, w: 20 }, Dataset::new(10, 20));
    }

    //noinspection DuplicatedCode
    #[test]
    fn test_dataset_is_end() {
        assert!(!Dataset::new(10, 20).is_end());
        assert!(!Dataset::new(0, 20).is_end());
        assert!(!Dataset::new(10, 0).is_end());
        assert!(Dataset::new(0, 0).is_end());
    }

    //noinspection DuplicatedCode
    #[test]
    fn test_dataset_from_str() {
        assert_eq!(Ok(Dataset { h: 3, w: 4 }), Dataset::from_str("3 4\n"));
        assert_eq!(Ok(Dataset { h: 5, w: 6 }), Dataset::from_str("5 6"));
        assert_eq!(Ok(Dataset { h: 2, w: 2 }), Dataset::from_str("2 2"));
        assert_eq!(Ok(Dataset { h: 0, w: 0 }), Dataset::from_str("0 0"));
    }

    #[test]
    fn test_create_rect() {
        assert_eq!("####\n#..#\n####", create_rect(&Dataset::new(3, 4)));
        assert_eq!("######\n#....#\n#....#\n#....#\n######", create_rect(&Dataset::new(5, 6)));
    }
}