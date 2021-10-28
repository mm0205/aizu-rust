//! ITP1_7_Cの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_7_C](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_7_C)

use std::io::BufRead;

#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some(result) = read_result(std::io::stdin().lock()) {
            for row in result.rows {
                for (i, n) in row.iter().enumerate() {
                    print_number(i, *n);
                }
                println!();
            }

            for (i, n) in result.totals.iter().enumerate() {
                print_number(i, *n);
            }
            println!();

            continue;
        }
        return;
    }
}

fn print_number(i: usize, n: i32) {
    if i > 0 {
        print!(" ");
    }
    print!("{}", n);
}


/// 回答。
#[derive(Debug, Eq, PartialEq)]
struct ResultSet {
    rows: Vec<Vec<i32>>,
    totals: Vec<i32>,
}

impl ResultSet {
    fn new(row_count: usize, col_count: usize) -> Self {
        let rows = vec![vec![0; col_count + 1]; row_count];
        ResultSet { rows, totals: vec![0; col_count + 1] }
    }

    fn set_row(&mut self, row_index: usize, v: &Vec<i32>) {
        let mut sum = 0;

        for i in 0..v.len() {
            let v_i = v[i];
            sum += v_i;
            self.totals[i] += v_i;
            self.rows[row_index][i] = v_i;
        }
        self.totals[v.len()] += sum;
        self.rows[row_index][v.len()] = sum;
    }
}

fn read_result<T: BufRead>(mut reader: T) -> Option<ResultSet> {
    if let Some((row_count, col_count)) = read_size(&mut reader) {
        let mut result = ResultSet::new(row_count, col_count);

        for row_index in 0..row_count {
            let row = read_row(&mut reader);
            result.set_row(row_index, &row);
        }

        return Some(result);
    }
    None
}

fn read_row<T: BufRead>(mut reader: T) -> Vec<i32> {
    let mut line = String::new();
    if let Ok(_) = reader.read_line(&mut line) {
        return line.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    }
    return vec![0; 0];
}

fn read_size<T: BufRead>(mut reader: T) -> Option<(usize, usize)> {
    let mut line = String::new();
    if let Err(_) = reader.read_line(&mut line) {
        return None;
    }

    let fields = line.trim().split(' ').map(|x| x.parse::<usize>());

    let mut numbers = Vec::new();
    for field in fields {
        let n = match field {
            Ok(n) => n,
            Err(_) => return None
        };
        numbers.push(n);
    }

    if numbers.len() != 2 {
        return None;
    }

    Some((numbers[0], numbers[1]))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_result_set_new() {
        let r = ResultSet::new(4, 5);
        assert_eq!(4, r.rows.len());
        assert_eq!(6, r.totals.len());
        assert!(r.rows.iter().all(|x| x.len() == 6));
    }

    #[test]
    fn test_result_set_add_row() {
        let mut rs = ResultSet::new(2, 3);
        rs.set_row(0, &vec![1, 2, 3]);
        rs.set_row(1, &vec![4, 5, 6]);
        assert_eq!(vec![1, 2, 3, 6], rs.rows[0]);
        assert_eq!(vec![4, 5, 6, 15], rs.rows[1]);
        assert_eq!(vec![5, 7, 9, 21], rs.totals);
    }

    #[test]
    fn test_read_result() {
        let input = Cursor::new(b"4 5\n1 1 3 4 5\n2 2 2 4 5\n3 3 0 1 1\n2 3 4 4 6\n");
        let result = read_result(input);
        assert_ne!(None, result);
        let rs = result.unwrap();
        assert_eq!(vec![1, 1, 3, 4, 5, 14], rs.rows[0]);
        assert_eq!(vec![2, 2, 2, 4, 5, 15], rs.rows[1]);
        assert_eq!(vec![3, 3, 0, 1, 1, 8], rs.rows[2]);
        assert_eq!(vec![2, 3, 4, 4, 6, 19], rs.rows[3]);
        assert_eq!(vec![8, 9, 9, 13, 17, 56], rs.totals);
    }
}