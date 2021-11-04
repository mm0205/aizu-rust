//! ITP1_9_Aの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_9_A](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_9_A)

use std::io::BufRead;

#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some(dataset) = Dataset::from_reader(std::io::stdin().lock()) {
            let number_of_w = dataset.count_w();
            println!("{}", number_of_w);
            continue;
        }
        return;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Dataset {
    w: String,
    t: Vec<String>,
}

impl Dataset {
    fn new(w: String) -> Self {
        Dataset { w, t: Vec::new() }
    }

    fn from_reader<T: BufRead>(mut reader: T) -> Option<Self> {
        let mut w = String::new();
        if let Err(_) = reader.read_line(&mut w) {
            return None;
        }
        let w = w.trim();
        if w.is_empty() {
            return None;
        }

        let mut result = Dataset::new(w.to_owned());

        loop {
            let mut line = String::new();
            if let Err(_) = reader.read_line(&mut line) {
                return None;
            }
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            if line == "END_OF_TEXT" {
                break;
            }
            result.t.push(line.to_owned());
        }

        Some(result)
    }

    fn count_w(&self) -> usize {
        self.t
            .iter()
            .flat_map(|x| x.split_whitespace())
            .filter(|x| self.w.eq_ignore_ascii_case(x))
            .count()
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_dataset_new() {
        assert_eq!(Dataset { w: "computer".to_owned(), t: Vec::new() }, Dataset::new("computer".to_owned()));
    }

    #[test]
    fn test_dataset_from_reader() {
        let input = Cursor::new("computer\n\
        Nurtures computer scientists and highly skilled computer engineers\n\
        who will create and exploit knowledge for the new era\n\
        Provides an outstanding computer environment\n\
        END_OF_TEXT");

        let t = vec![
            "Nurtures computer scientists and highly skilled computer engineers".to_owned(),
            "who will create and exploit knowledge for the new era".to_owned(),
            "Provides an outstanding computer environment".to_owned(),
        ];

        assert_eq!(Some(Dataset { w: "computer".to_owned().to_owned(), t }), Dataset::from_reader(input));
    }

    #[test]
    fn test_split_whitespace() {
        let t = vec!["a b   c".to_owned(), "ab c".to_owned()];
        let _v: Vec<&str> = t.iter().flat_map(|x| x.split_whitespace()).collect();
        assert_eq!(vec!["a", "b", "c", "ab", "c"], _v);
    }

    #[test]
    fn test_dataset_count_w() {
        let input = Cursor::new("computer\n\
        Nurtures computer scientists and highly skilled computer engineers\n\
        who will create and exploit knowledge for the new era\n\
        Provides an outstanding computer environment\n\
        END_OF_TEXT");
        assert_eq!(3, Dataset::from_reader(input).unwrap().count_w());
    }

    #[test]
    fn test_input_2() {
        let input = Cursor::new("aizu\n\
aaa aizu xxx xx x xxx aizu y yyyy a a a a a a aizu\n\
aaa aa xxx WWW AAA aize aiza AIZZ AAA III a i z u aizu aizu aizu\n\
SSS SS S aizu AAAAAAA a a a a a x x x x aizu zia uzia uzia aizu aizu aizu aizu\n\
Aizu hyoooon pupupupu ohohohoh\n\
aa eee fff eee fff ee f f f f f f f AIZU bebebebebebe ai zu a izu aiz u\n\
Aizu aizu Aizu AIzU\n\
end_of_text\n\
END_OF_TEXP\n\
aizu Aizu\n\
END_OF_TEXT");
        assert_eq!(20, Dataset::from_reader(input).unwrap().count_w());
    }
}