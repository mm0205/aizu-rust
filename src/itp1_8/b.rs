//! ITP1_8_Bの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_8_B](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_8_B)

use std::io::BufRead;

/// ITP1_8_Bの回答
#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some(numbers) = read_numbers(std::io::stdin().lock()) {
            println!("{}", numbers.iter().sum::<i32>());
            continue;
        }
        return;
    }
}

fn read_numbers<T: BufRead>(mut reader: T) -> Option<Vec<i32>> {
    let mut line = String::new();
    if let Err(_) = reader.read_line(&mut line) {
        return None;
    }

    let mut result = Vec::new();
    for c in line.trim().chars() {
        if let Ok(n) = c.to_string().parse::<i32>() {
            result.push(n);
        } else {
            return None;
        }
    }
    if result.len() == 1 && result[0] == 0 {
        return None;
    }

    Some(result)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_read_numbers() {
        assert_eq!(Some(vec![1, 2, 3]), read_numbers(Cursor::new("123\n")));
        assert_eq!(Some(vec![5, 5]), read_numbers(Cursor::new("55\n")));
        assert_eq!(Some(vec![1, 0, 0, 0]), read_numbers(Cursor::new("1000\n")));
        assert_eq!(None, read_numbers(Cursor::new("0\n")));
    }
}