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

            let dataset = line_to_dataset(line);
            if is_end_of_input(dataset) {
                return;
            }
            answer(dataset);
        } else {
            return;
        }
    }
}

fn line_to_dataset(line: &str) -> (i32, i32) {
    let mut numbers: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
    numbers.sort();
    (numbers[0], numbers[1])
}

fn is_end_of_input(dataset: (i32, i32)) -> bool {
    match dataset {
        (0, 0) => true,
        _ => false
    }
}

fn answer(dataset: (i32, i32)) {
    println!("{} {}", dataset.0, dataset.1)
}