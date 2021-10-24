//! ITP1_5_Dの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_5_D](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_5_D)


/// ITP1_5_Dの回答
#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut line) {
            return;
        }
        if let Ok(n) = line.trim().parse::<i32>() {
            println!("{}", compute_check_sum(n));
            continue;
        }
        return;
    }
}

fn compute_check_sum(n: i32) -> String {
    let mut numbers = Vec::new();

    for i in 1..=n {
        let mut x = i;
        if x % 3 == 0 {
            numbers.push(i.to_string());
            continue;
        }

        while x > 0 {
            if x % 10 == 3 {
                numbers.push(i.to_string());
                break;
            }
            x /= 10
        }
    }

    numbers.join(" ")
}

#[cfg(test)]
mod test {
    use crate::itp1_5::d::compute_check_sum;

    #[test]
    fn test_compute_check_sum() {
        assert_eq!("3 6 9 12 13 15 18 21 23 24 27 30", compute_check_sum(30))
    }
}