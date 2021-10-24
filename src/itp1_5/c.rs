//! ITP1_5_Cの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_5_C](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_5_C)

use std::io::stdin;
use std::str::FromStr;

//noinspection DuplicatedCode
/// ITP1_5_Cの回答
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
    let src = vec!['#', '.'];
    let mut result = String::new();

    let mut y_offset = 0;
    for y in 0..dataset.h {
        if y > 0 {
            result.push('\n');
        }
        let mut x_offset = 0;
        for _ in 0..dataset.w {
            let c = src[(x_offset + y_offset) % 2];
            result.push(c);
            x_offset += 1;
        }
        y_offset += 1;
    }
    result.to_string()
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
        assert_eq!("#.#.\n.#.#\n#.#.", create_rect(&Dataset::new(3, 4)));
        assert_eq!("#.#.#.\n.#.#.#\n#.#.#.\n.#.#.#\n#.#.#.", create_rect(&Dataset::new(5, 6)));
    }
}