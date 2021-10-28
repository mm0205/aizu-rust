//! ITP1_7_Dの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_7_D](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_7_D)


use std::io::BufRead;

/// ITP1_7_Dの回答
#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some((a, b)) = read_input(std::io::stdin().lock()) {
            let c = mul_mat(&a, &b);
            for row in c {
                for (i, v) in row.iter().enumerate() {
                    if i > 0 {
                        print!(" ");
                    }
                    print!("{}", v)
                }
                println!();
            }
            continue;
        }
        return;
    }
}

fn read_numbers<T: BufRead>(mut reader: T) -> Option<Vec<i64>> {
    let mut line = String::new();
    if let Err(_) = reader.read_line(&mut line) {
        return None;
    }

    let mut result = Vec::new();
    for n in line.trim().split(' ').map(|x| x.parse::<i64>()) {
        if let Ok(n) = n {
            result.push(n)
        } else {
            return None;
        }
    }
    Some(result)
}

fn read_input<T: BufRead>(mut reader: T) -> Option<(Vec<Vec<i64>>, Vec<Vec<i64>>)> {
    if let Some(nml) = read_numbers(&mut reader) {
        let a = read_matrix(&mut reader, nml[0]);
        let b = read_matrix(&mut reader, nml[1]);
        return match (a, b) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None
        };
    }
    None
}

fn read_matrix<T: BufRead>(mut reader: T, count: i64) -> Option<Vec<Vec<i64>>> {
    let mut a = Vec::new();
    for _ in 0..count {
        if let Some(v) = read_numbers(&mut reader) {
            a.push(v);
        } else {
            return None;
        }
    }
    Some(a)
}

fn mul_mat(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut c = vec![vec![0; b[0].len()]; a.len()];

    for i in 0..c.len() {
        let a_i = &a[i];
        for j in 0..c[i].len() {
            for k in 0..a_i.len() {
                c[i][j] += a_i[k] * b[k][j];
            }
        }
    }
    c
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_read_numbers() {
        assert_eq!(Some(vec![3, 2, 3]), read_numbers(Cursor::new("3 2 3\n1 2\n")));
    }

    #[test]
    fn test_read_matrix() {
        assert_eq!(Some(vec![vec![1, 2], vec![0, 3], vec![4, 5]]), read_matrix(Cursor::new(b"1 2\n0 3\n4 5"), 3));
        assert_eq!(Some(vec![vec![1, 2, 1], vec![0, 3, 2]]), read_matrix(Cursor::new(b"1 2 1\n0 3 2\n"), 2));
    }

    #[test]
    fn test_read_input() {
        assert_eq!(Some((vec![vec![1, 2], vec![0, 3], vec![4, 5]], vec![vec![1, 2, 1], vec![0, 3, 2]])),
                   read_input(Cursor::new(b"3 2 3\n1 2\n0 3\n4 5\n1 2 1\n0 3 2\n")));
    }

    #[test]
    fn test_mul_mat() {
        let (a, b) = read_input(Cursor::new(b"3 2 3\n1 2\n0 3\n4 5\n1 2 1\n0 3 2\n")).unwrap();
        assert_eq!(vec![vec![1, 8, 5], vec![0, 9, 6], vec![4, 23, 14]], mul_mat(&a, &b));
    }
}