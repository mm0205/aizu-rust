//! ITP1_10_Cの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_10_C](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_10_C)

use std::io::BufRead;

/// ITP1_10_C
#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some(dataset) = read_dataset(std::io::stdin().lock()) {
            let result = compute_std_dev(&dataset);
            println!("{}", result);
            continue;
        }
        break;
    }
}

fn read_dataset<T: BufRead>(mut reader: T) -> Option<Vec<i32>> {
    let mut l1 = String::new();
    if let Err(_) = reader.read_line(&mut l1) {
        return None;
    }

    if 0 == l1.trim().parse().unwrap() {
        return None;
    }

    let mut l2 = String::new();
    reader.read_line(&mut l2).unwrap();
    Some(l2.trim().split(' ').map(|x| x.parse().unwrap()).collect())
}

fn compute_std_dev(dataset: &[i32]) -> f64 {
    let n = dataset.len() as f64;

    let m = dataset.iter()
        .map(|x| *x as f64)
        .sum::<f64>() as f64 / n;

    let v = dataset.iter().fold(0f64, |a, b| {
        let temp = *b as f64 - m;
        a + temp.powi(2)
    }) / n;
    v.sqrt()
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    fn near(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.0001
    }

    #[test]
    fn test_read_dataset() {
        let input = Cursor::new("5\n70 80 100 90 20\n");
        assert_eq!(Some(vec![70, 80, 100, 90, 20]), read_dataset(input));
    }

    #[test]
    fn test_read_dataset_end() {
        let input = Cursor::new("0");
        assert_eq!(None, read_dataset(input));
    }

    #[test]
    fn test_compute_std_dev() {
        assert!(near(27.85677655, compute_std_dev(&vec![70, 80, 100, 90, 20])));
    }

    #[test]
    fn test_compute_std_dev_2() {
        assert!(near(0f64, compute_std_dev(&vec![80, 80, 80])));
    }
}