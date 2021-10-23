//! ITP1_4_Dの回答。
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_4_D](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_4_D)

use std::io::BufRead;

/// ITP1_4_Dの回答(エントリポイント)。
#[allow(dead_code)]
pub fn main() {
    loop {
        if let None = read_input(std::io::stdin().lock()) {
            return;
        }
        if let Some(input) = read_input(std::io::stdin().lock()) {
            let result = compute_result(input);
            println!("{} {} {}", result.min, result.max, result.sum);
            continue;
        }

        return;
    }
}

fn read_input<T: BufRead>(mut read: T) -> Option<Vec<i32>> {
    let mut line = String::new();

    if let Err(_) = read.read_line(&mut line) {
        return None;
    }

    let mut result = Vec::new();
    for x in line.trim().split(' ').map(|x| x.parse::<i32>()) {
        match x {
            Ok(n) => result.push(n),
            _ => return None
        }
    }

    Some(result)
}

#[derive(Debug, PartialEq)]
struct ResultSet {
    min: i32,
    max: i32,
    sum: i64,
}

fn compute_result(v: Vec<i32>) -> ResultSet {
    v.iter().fold(ResultSet { min: 1000000, max: -1000000, sum: 0 }, |result, current| {
        return ResultSet {
            min: std::cmp::min(result.min, *current),
            max: std::cmp::max(result.max, *current),
            sum: result.sum + (*current as i64),
        };
    })
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_read_input() {
        let mut input = Cursor::new(b"5\r\n10 1 5 4 17");
        assert_eq!(vec![5], read_input(&mut input).unwrap());
        assert_eq!(vec![10, 1, 5, 4, 17], read_input(&mut input).unwrap());
    }

    #[test]
    fn test_compute_result() {
        assert_eq!(ResultSet { min: 1, max: 5, sum: 15 }, compute_result(vec![1, 2, 3, 4, 5]));
        assert_eq!(ResultSet { min: 1, max: 17, sum: 37 }, compute_result(vec![10, 1, 5, 4, 17]));
    }
}