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
    let mut numbers: Vec<i32> = line.split(' ')
        .map(|x| x.parse().unwrap()).collect();

    numbers.sort();

    print!("{}", numbers[0]);
    numbers.iter().skip(1).for_each(|x| {
        print!(" {}", x);
    });
    println!();
}