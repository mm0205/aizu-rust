//! ITP1_6_Aの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_6_A](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_6_A)

use std::io::BufRead;

#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some(input) = read_input(std::io::stdin().lock()) {
            let result = compute_reverse(&input);
            println!("{}", result);
        }

        return;
    }
}

fn read_input<T: BufRead>(mut reader: T) -> Option<String> {
    let mut line = String::new();
    if let Err(_) = reader.read_line(&mut line) {
        return None;
    }

    let mut line = String::new();
    if let Err(_) = reader.read_line(&mut line) {
        return None;
    }
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    Some(line.trim().to_string())
}

fn compute_reverse(numbers: &str) -> String {
    numbers.split(' ')
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_read_input() {
        let input = Cursor::new(b"5\n1 2 3 4 5");
        assert_eq!(Some("1 2 3 4 5".to_string()), read_input(input));
    }

    #[test]
    fn test_compute() {
        assert_eq!("5 4 3 2 1", compute_reverse("1 2 3 4 5"));
        assert_eq!("9 7 8 5 4 4 3 3", compute_reverse("3 3 4 4 5 8 7 9"));
        assert_eq!("900 7 8 5 4 4 3 3", compute_reverse("3 3 4 4 5 8 7 900"));
    }
}