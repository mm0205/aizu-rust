use std::cmp::Ordering;
use std::io;

#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Ok(_) = io::stdin().read_line(&mut line) {
            let line = line.trim();
            if line.is_empty() {
                return;
            }

            answer(line);
        } else {
            return;
        }
    }
}

fn answer(line: &str) {
    let fields: Vec<&str> = line.split(" ").collect();
    let a: i32 = fields[0].parse().unwrap();
    let b: i32 = fields[1].parse().unwrap();

    let output = match a.cmp(&b) {
        Ordering::Less => "a < b",
        Ordering::Equal => "a == b",
        Ordering::Greater => "a > b"
    };
    println!("{}", output);
}