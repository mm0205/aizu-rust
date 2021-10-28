//! ITP1_8_Aの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_8_A](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_8_A)

use std::io::BufRead;

#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some(line) = read_line(std::io::stdin().lock()) {
            for c in line.chars() {
                if c.is_uppercase() {
                    print!("{}", c.to_lowercase())
                } else if c.is_lowercase() {
                    print!("{}", c.to_uppercase())
                } else {
                    print!("{}", c)
                }
            }
            println!();
            continue;
        }
        return;
    }
}

fn read_line<T: BufRead>(mut reader: T) -> Option<String> {
    let mut line = String::new();
    if let Err(_) = reader.read_line(&mut line) {
        return None;
    }
    line = line.trim().to_string();
    if line.is_empty() {
        return None
    }
    Some(line)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_read_line() {
        assert_eq!(Some("fAIR, LATER, OCCASIONALLY CLOUDY.".to_string()), read_line(Cursor::new(b"fAIR, LATER, OCCASIONALLY CLOUDY.")));
    }

    #[test]
    fn test_c() {
        assert_eq!(2, "𩸽😅".chars().collect::<Vec<char>>().len());
    }
}
