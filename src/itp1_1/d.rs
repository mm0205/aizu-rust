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

            answer(line)
        } else {
            return;
        }
    }
}

struct Hms {
    h: i32,
    m: i32,
    s: i32,
}

fn answer(line: &str) {
    let second: i32 = line.parse().unwrap();
    let hms = compute_hms_from_sec(second);
    println!("{}:{}:{}", hms.h, hms.m, hms.s);
}

fn compute_hms_from_sec(second: i32) -> Hms {
    let h = second / (60 * 60);
    let second = second - h * 60 * 60;
    let m = second / 60;
    let s = second - m * 60;
    Hms {
        h,
        m,
        s,
    }
}