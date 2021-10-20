use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut n = 1;
    loop {
        let mut line = String::new();
        if let Ok(_) = io::stdin().read_line(&mut line) {
            let line = line.trim();
            if line.is_empty() {
                return;
            }
            let x: i32 = line.parse().unwrap();
            if x == 0 {
                return;
            }

            println!("Case {}: {}", n, x);
            n += 1;
        } else {
            return;
        }
    }
}