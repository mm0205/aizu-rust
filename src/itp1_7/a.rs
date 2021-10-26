//! ITP1_7_Aの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/finder.jsp?course=ITP1](https://judge.u-aizu.ac.jp/onlinejudge/finder.jsp?course=ITP1)

use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut line) {
            return;
        }
        if let Ok(s) = ScoreSet::from_str(&line) {
            if s.is_end() {
                return;
            }
            println!("{}", s.evaluate());
            continue;
        }
        return;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ScoreSet {
    midterm_exam: i32,
    final_exam: i32,
    makeup_exam: i32,
}

impl ScoreSet {
    fn new(midterm_exam: i32, final_exam: i32, makeup_exam: i32) -> Self {
        Self { midterm_exam, final_exam, makeup_exam }
    }

    fn is_end(&self) -> bool {
        self.midterm_exam == -1
            && self.final_exam == -1
            && self.makeup_exam == -1
    }

    fn evaluate(&self) -> Performance {
        match self {
            &Self { midterm_exam: -1, final_exam: _, .. } => Performance::F,
            &Self { midterm_exam: _, final_exam: -1, .. } => Performance::F,
            &Self { midterm_exam, final_exam, .. } if midterm_exam + final_exam >= 80 => Performance::A,
            &Self { midterm_exam, final_exam, .. } if midterm_exam + final_exam >= 65 => Performance::B,
            &Self { midterm_exam, final_exam, .. } if midterm_exam + final_exam >= 50 => Performance::C,
            &Self { midterm_exam, final_exam, .. } if midterm_exam + final_exam < 30 => Performance::F,
            &Self { makeup_exam, .. } if makeup_exam >= 50 => Performance::C,
            _ => Performance::D,
        }
    }
}

impl FromStr for ScoreSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec = vec![0; 3];
        for (i, n) in s.trim().split(' ').map(|x| x.parse()).enumerate() {
            vec[i] = match n {
                Ok(n) => n,
                _ => return Err(())
            };
        }
        Ok(Self::new(vec[0], vec[1], vec[2]))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Performance {
    A,
    B,
    C,
    D,
    F,
}

impl Display for Performance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Performance::A => "A",
            Performance::B => "B",
            Performance::C => "C",
            Performance::D => "D",
            Performance::F => "F",
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_performance_rule_1_f() {
        assert_eq!(Performance::F, ScoreSet::new(-1, 50, 100).evaluate());
        assert_eq!(Performance::F, ScoreSet::new(50, -1, 100).evaluate());
    }

    #[test]
    fn test_performance_rule_2_a() {
        assert_eq!(Performance::A, ScoreSet::new(50, 30, -1).evaluate());
        assert_eq!(Performance::A, ScoreSet::new(30, 50, 50).evaluate());
    }

    #[test]
    fn test_performance_rule_3_b() {
        assert_eq!(Performance::B, ScoreSet::new(32, 33, -1).evaluate());
        assert_eq!(Performance::B, ScoreSet::new(15, 50, 50).evaluate());
    }

    #[test]
    fn test_performance_rule_4_c() {
        assert_eq!(Performance::C, ScoreSet::new(25, 25, -1).evaluate());
        assert_eq!(Performance::C, ScoreSet::new(0, 50, 50).evaluate());
        assert_eq!(Performance::C, ScoreSet::new(50, 0, 50).evaluate());
    }

    #[test]
    fn test_performance_rule_5_c_d() {
        assert_eq!(Performance::D, ScoreSet::new(0, 30, -1).evaluate());
        assert_eq!(Performance::D, ScoreSet::new(30, 0, 49).evaluate());
        assert_eq!(Performance::D, ScoreSet::new(15, 15, 15).evaluate());

        assert_eq!(Performance::C, ScoreSet::new(15, 15, 50).evaluate());
    }

    #[test]
    fn test_performance_rule_6_f() {
        assert_eq!(Performance::F, ScoreSet::new(0, 29, 100).evaluate());
        assert_eq!(Performance::F, ScoreSet::new(29, 0, 50).evaluate());
        assert_eq!(Performance::F, ScoreSet::new(15, 14, -1).evaluate());
    }

    #[test]
    fn test_score_set_from_str() {
        assert_eq!(Ok(ScoreSet::new(40, 42, -1)), ScoreSet::from_str("40 42 -1\n"));
        assert_eq!(Ok(ScoreSet::new(20, 30, -1)), ScoreSet::from_str("20 30 -1\n"));
        assert_eq!(Ok(ScoreSet::new(0, 2, -1)), ScoreSet::from_str("0 2 -1\n"));
        assert_eq!(Ok(ScoreSet::new(-1, -1, -1)), ScoreSet::from_str("-1 -1 -1\n"));
    }

    #[test]
    fn test_score_set_is_end() {
        assert!(ScoreSet::new(-1, -1, -1).is_end());
        for a in 0..=50 {
            for b in 0..=50 {
                for c in 0..100 {
                    assert!(!ScoreSet::new(a, b, c).is_end());
                }
            }
        }
    }

    #[test]
    fn test_format_performance() {
        assert_eq!("A", format!("{}", Performance::A));
        assert_eq!("B", format!("{}", Performance::B));
        assert_eq!("C", format!("{}", Performance::C));
        assert_eq!("D", format!("{}", Performance::D));
        assert_eq!("F", format!("{}", Performance::F));
    }
}