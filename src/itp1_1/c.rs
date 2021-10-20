use std::io;

#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        if let Ok(_) = io::stdin().read_line(&mut line) {
            let line = line.trim();
            if line.len() <= 0 {
                return
            }
            compute_area_and_perimeter(line);
        } else {
            return;
        }
    }
}

fn compute_area_and_perimeter(line: &str) {
    let lengths: Vec<&str> = line.split(" ").collect();
    let a: i32 = lengths[0].parse().unwrap();
    let b: i32 = lengths[1].parse().unwrap();

    let area = a * b;
    let perimeter = a * 2 + b * 2;

    println!("{} {}", area, perimeter)
}