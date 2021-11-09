//! ITP1_10_Dの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_10_D](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_10_D)

use std::io::BufRead;

/// ITP1_10_Dの回答
#[allow(dead_code)]
pub fn main() {
    let dataset = Dataset::read(std::io::stdin().lock());
    let answer = dataset.compute();
    println!("{}", answer.p_1);
    println!("{}", answer.p_2);
    println!("{}", answer.p_3);
    println!("{}", answer.p_inf);
}

#[derive(Debug, Eq, PartialEq)]
struct Dataset {
    xs: Vec<i32>,
    ys: Vec<i32>,
}

impl Dataset {
    fn read<T: BufRead>(mut reader: T) -> Self {
        let _n = Self::read_n(&mut reader);
        let xs = Self::read_vec(&mut reader);
        let ys = Self::read_vec(&mut reader);
        Dataset { xs, ys }
    }

    fn compute(&self) -> Answer {
        let mut answer = Answer::new();

        for i in 0..self.xs.len() {
            let x = self.xs[i] as f64;
            let y = self.ys[i] as f64;
            answer.add_item(x, y);
        }

        answer.fix();
        answer
    }

    fn read_line<T: BufRead>(mut reader: T) -> String {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        line
    }

    fn read_n<T: BufRead>(mut reader: T) -> i32 {
        Self::read_line(&mut reader).trim().parse().unwrap()
    }

    fn read_vec<T: BufRead>(mut reader: T) -> Vec<i32> {
        Self::read_line(&mut reader)
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect()
    }
}

#[derive(Debug)]
struct Answer {
    p_1: f64,
    p_2: f64,
    p_3: f64,
    p_inf: f64,
}

impl PartialEq for Answer {
    fn eq(&self, other: &Self) -> bool {
        let min_error = 0.0001;
        (self.p_1 - other.p_1).abs() < min_error
            && (self.p_2 - other.p_2).abs() < min_error
            && (self.p_3 - other.p_3).abs() < min_error
            && (self.p_inf - other.p_inf).abs() < min_error
    }
}

impl Answer {
    fn new() -> Self {
        Answer { p_1: 0f64, p_2: 0f64, p_3: 0f64, p_inf: 0f64 }
    }

    fn add_item(&mut self, x: f64, y: f64) {
        let item = (x - y).abs();
        self.p_1 += item;
        self.p_2 += item.powi(2);
        self.p_3 += item.powi(3);
        self.p_inf = if self.p_inf >= item { self.p_inf } else { item };
    }

    fn fix(&mut self) {
        self.p_2 = self.p_2.sqrt();
        self.p_3 = self.p_3.powf(1f64 / 3f64);
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_read_dataset() {
        let dataset = Dataset::read(Cursor::new("3\n1 2 3\n2 0 4\n"));
        assert_eq!(Dataset { xs: vec![1, 2, 3], ys: vec![2, 0, 4] }, dataset);
    }

    #[test]
    fn test_compute_answer() {
        let dataset = Dataset::read(Cursor::new("3\n1 2 3\n2 0 4\n"));
        let answer = dataset.compute();
        assert_eq!(Answer { p_1: 4f64, p_2: 2.449490, p_3: 2.154435, p_inf: 2f64 }, answer);
    }

    #[test]
    fn test_answer_add() {
        let mut answer = Answer::new();
        answer.add_item(0f64, 2f64);
        assert_eq!(Answer { p_1: 2f64, p_2: 4f64, p_3: 8f64, p_inf: 2f64 }, answer);
    }

    #[test]
    fn test_answer_add_2() {
        let mut answer = Answer::new();
        answer.add_item(0f64, 2f64);
        answer.add_item(3f64, 1f64);
        assert_eq!(Answer { p_1: 4f64, p_2: 8f64, p_3: 16f64, p_inf: 2f64 }, answer);
    }

    #[test]
    fn test_answer_fix() {
        let mut answer = Answer::new();
        answer.add_item(0f64, 2f64);
        answer.add_item(3f64, 1f64);
        answer.fix();
        assert_eq!(Answer { p_1: 4f64, p_2: 8f64.sqrt(), p_3: 16f64.powf(1f64 / 3f64), p_inf: 2f64 }, answer);
    }
}

