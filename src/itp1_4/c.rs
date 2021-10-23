//! ITP1_4_Cの回答。
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_4_C](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_4_C)

use std::str::FromStr;

/// ITP1_4_Cの回答(エントリポイント)。
pub fn main() {
    loop {
        let input = read_input();
        if let None = input {
            return;
        }
        if let Ok(dataset) = Dataset::from_str(&input.unwrap()) {
            if let Some(result) = compute_result(&dataset) {
                println!("{}", result);
                continue;
            }
        }

        return;
    }
}

fn read_input() -> Option<String> {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => Some(line),
        _ => None
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Sum,
    Difference,
    Product,
    Quotient,
    End,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Sum),
            "-" => Ok(Operator::Difference),
            "*" => Ok(Operator::Product),
            "/" => Ok(Operator::Quotient),
            "?" => Ok(Operator::End),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
struct Dataset {
    a: i32,
    op: Operator,
    b: i32,
}

impl FromStr for Dataset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s.trim().split(' ').collect::<Vec<&str>>();
        if fields.len() != 3 {
            return Err(());
        }

        let a = get_i32(fields[0])?;
        let op = Operator::from_str(fields[1])?;
        let b = get_i32(fields[2])?;

        Ok(Dataset { a, op, b })
    }
}


fn get_i32(i32_text: &str) -> Result<i32, ()> {
    match i32_text.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(()),
    }
}

fn compute_result(dataset: &Dataset) -> Option<i32> {
    match dataset.op {
        Operator::Sum => Some(dataset.a + dataset.b),
        Operator::Difference => Some(dataset.a - dataset.b),
        Operator::Product => Some(dataset.a * dataset.b),
        Operator::Quotient => Some(dataset.a / dataset.b),
        Operator::End => None
    }
}

#[cfg(test)]
mod tes {
    use super::*;

    #[test]
    fn test_dataset_from_str() {
        assert_eq!(Ok(Dataset { a: 1, op: Operator::Sum, b: 2 }), Dataset::from_str("1 + 2\r\n"));
        assert_eq!(Ok(Dataset { a: 56, op: Operator::Difference, b: 18 }), Dataset::from_str("56 - 18"));
        assert_eq!(Ok(Dataset { a: 13, op: Operator::Product, b: 2 }), Dataset::from_str("13 * 2\r\n"));
        assert_eq!(Ok(Dataset { a: 100, op: Operator::Quotient, b: 10 }), Dataset::from_str("100 / 10\r\n"));
        assert_eq!(Ok(Dataset { a: 27, op: Operator::Sum, b: 81 }), Dataset::from_str("27 + 81\r\n"));
        assert_eq!(Ok(Dataset { a: 0, op: Operator::End, b: 0 }), Dataset::from_str("0 ? 0\r\n"));

        assert_eq!(Err(()), Dataset::from_str(""));
    }

    #[test]
    fn test_compute_result() {
        assert_eq!(Some(3), compute_result(&Dataset::from_str("1 + 2\r\n").unwrap()));
        assert_eq!(Some(38), compute_result(&Dataset::from_str("56 - 18").unwrap()));
        assert_eq!(Some(26), compute_result(&Dataset::from_str("13 * 2\r\n").unwrap()));
        assert_eq!(Some(10), compute_result(&Dataset::from_str("100 / 10\r\n").unwrap()));
        assert_eq!(Some(108), compute_result(&Dataset::from_str("27 + 81\r\n").unwrap()));
        assert_eq!(None, compute_result(&Dataset::from_str("0 ? 0\r\n").unwrap()));
    }
}