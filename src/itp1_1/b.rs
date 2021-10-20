use std::io;

#[allow(dead_code)]
pub fn main() {
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => match print_cube_of(line) {
                Some(_) => (),
                _ => return
            },
            _ => return
        }
    }
}

fn print_cube_of(input_text: String) -> Option<i32> {
    let trimmed = input_text.trim();
    if trimmed.len() <= 0 {
        return None;
    }
    let num: i32 = input_text.trim().parse().unwrap();
    println!("{}", num.pow(3));
    Some(num)
}
