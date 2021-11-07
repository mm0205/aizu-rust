//! ITP1_9_Bの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_9_B](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_9_B)

use std::io::BufRead;

/// ITP1_9_Bの回答
#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some(mut dataset) = Dataset::from_optional_string(read_string(std::io::stdin().lock())) {
            do_shuffles(std::io::stdin().lock(), &mut dataset);

            println!("{}", dataset.s);
            continue;
        }
        return;
    }
}


fn read_string<T: BufRead>(mut reader: T) -> Option<String> {
    let mut line = String::new();

    if let Err(_) = reader.read_line(&mut line) {
        return None;
    }

    let line = line.trim();
    if line.is_empty() || line == "-" {
        return None;
    }

    Some(line.to_owned())
}

fn read_usize<T: BufRead>(mut reader: T) -> usize {
    read_string(&mut reader)
        .unwrap()
        .parse()
        .unwrap()
}

fn do_shuffles<T: BufRead>(mut reader: T, dataset: &mut Dataset) {
    let m = read_usize(&mut reader);
    for _ in 0..m {
        dataset.shuffle(read_usize(&mut reader));
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Dataset {
    s: String,
}

impl Dataset {
    fn new(s: String) -> Self {
        Dataset { s }
    }

    fn from_optional_string(s: Option<String>) -> Option<Self> {
        Some(Self::new(s?))
    }

    fn shuffle(&mut self, n: usize) {
        let (a, b) = self.s.split_at(n);
        self.s = b.to_owned() + a;
    }
}


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    //noinspection SpellCheckingInspection
    #[test]
    fn test_read_string() {
        assert_eq!(Some("aabc".to_owned()), read_string(Cursor::new("aabc")));
        assert_eq!(Some("vwxyz".to_owned()), read_string(Cursor::new("vwxyz")));
        assert_eq!(None, read_string(Cursor::new("-")));
    }

    #[test]
    //noinspection SpellCheckingInspection
    fn test_dataset_from_optional_str() {
        assert_eq!(Some(Dataset { s: "aabc".to_owned() }), Dataset::from_optional_string(Some("aabc".to_owned())));
        assert_eq!(Some(Dataset { s: "vwxyz".to_owned() }), Dataset::from_optional_string(Some("vwxyz".to_owned())));
        assert_eq!(None, Dataset::from_optional_string(None));
    }

    #[test]
    //noinspection SpellCheckingInspection
    fn test_dataset_shuffle_1() {
        let mut t = Dataset::new("aabc".to_owned());
        let target = &mut t;
        target.shuffle(1);
        target.shuffle(2);
        target.shuffle(1);
        assert_eq!("aabc", &target.s);
    }

    #[test]
    //noinspection SpellCheckingInspection
    fn test_dataset_shuffle_2() {
        let mut t = Dataset::new("vwxyz".to_owned());
        let target = &mut t;
        target.shuffle(3);
        target.shuffle(4);
        assert_eq!("xyzvw", &target.s);
    }

    #[test]
    //noinspection SpellCheckingInspection
    fn test_read_usize() {
        let mut input = Cursor::new("3\n1\n2\n1\n");
        assert_eq!(3, read_usize(&mut input));
        assert_eq!(1, read_usize(&mut input));
        assert_eq!(2, read_usize(&mut input));
        assert_eq!(1, read_usize(&mut input));
    }

    #[test]
    //noinspection SpellCheckingInspection
    fn test_do_shuffle() {
        let mut dataset = Dataset::new("vwxyz".to_owned());
        let input = Cursor::new("2\n3\n4\n");
        do_shuffles(input, &mut dataset);
        assert_eq!("xyzvw", &dataset.s);
    }
}