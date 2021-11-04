//! ITP1_8_Dの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_8_D](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_8_D)

use std::io::BufRead;

/// ITP1_8_Dの回答
#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some(dataset) = read_dataset(std::io::stdin().lock()) {
            let output = match dataset.exists_pattern() {
                true => "Yes",
                false => "No"
            };
            println!("{}", output);
            continue;
        }
        return;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Dataset {
    s: String,
    p: String,
}

impl Dataset {
    fn new(s: &str, p: &str) -> Self {
        Self { s: s.to_string(), p: p.to_string() }
    }

    fn exists_pattern(&self) -> bool {
        let temp = self.s.repeat(2);
        match temp.find(&self.p) {
            Some(_) => true,
            _ => false
        }
    }
}

fn read_dataset<T: BufRead>(mut reader: T) -> Option<Dataset> {
    let mut s = String::new();
    if let Err(_) = reader.read_line(&mut s) {
        return None;
    }
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let mut p = String::new();
    if let Err(_) = reader.read_line(&mut p) {
        return None;
    }
    if p.is_empty() {
        return None;
    }

    let p = p.trim();

    Some(Dataset::new(s, p))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    //noinspection SpellCheckingInspection
    #[test]
    fn test_dataset_new() {
        assert_eq!(Dataset { s: "vanceknowledgetoad".to_string(), p: "advance".to_string() },
                   Dataset::new("vanceknowledgetoad", "advance"));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_exists_pattern() {
        let dataset = Dataset::new("vanceknowledgetoad", "advance");
        assert!(dataset.exists_pattern())
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_read_dataset() {
        let mut input = Cursor::new("vanceknowledgetoad\nadvance\n");
        assert_eq!(Some(Dataset { s: "vanceknowledgetoad".to_string(), p: "advance".to_string() }),
                   read_dataset(&mut input));
    }
}