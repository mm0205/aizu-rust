//! ITP1_7_B の回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_7_B](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_7_B)

use std::io::BufRead;

#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some(numbers) = read_numbers(std::io::stdin().lock()) {
            if numbers[0] == 0 && numbers[1] == 0 {
                return;
            }

            let result = find_combination_of_three_numbers(numbers[0], numbers[1]);
            println!("{}", result);
            continue;
        }
        return;
    }
}

fn find_combination_of_three_numbers(n: i32, x: i32) -> i32 {
    let last = n;
    let penultimate = n - 1;
    let first = std::cmp::max(1, x - last - penultimate);
    let i_last = penultimate - 1;

    let mut sum = 0;
    for i in first..=i_last {
        let j_first = std::cmp::max(i + 1, x - i - last);
        for j in j_first..=penultimate {
            let k_first = std::cmp::max(j + 1, x - i - j);
            for k in k_first..=last {
                if i + j + k == x {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn read_numbers<T: BufRead>(mut reader: T) -> Option<Vec<i32>> {
    let mut line = String::new();
    if let Err(_) = reader.read_line(&mut line) {
        return None;
    }

    let mut result = vec![0; 2];
    for (i, n) in line.trim().split(' ').map(|x| x.parse::<i32>()).enumerate() {
        match n {
            Ok(n) => result[i] = n,
            _ => return None
        };
    }
    Some(result)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_find_combination_of_three_numbers() {
        assert_eq!(2, find_combination_of_three_numbers(5, 9));
    }

    #[test]
    fn test_read_numbers() {
        let mut input = Cursor::new(b"5 9\n0 0");
        assert_eq!(Some(vec![5, 9]), read_numbers(&mut input));
        assert_eq!(Some(vec![0, 0]), read_numbers(&mut input));
    }
}

