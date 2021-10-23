//! ITP1_2_Dの回答。
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_2_D](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_2_D)
use std::io;

/// ITP1_2_Dの回答(エントリポイント)。
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

struct Rect {
    left: i32,
    bottom: i32,
    width: i32,
    height: i32,
}

impl Rect {
    fn new_from_origin(width: i32, height: i32) -> Rect {
        return Rect::new(0, 0, width, height);
    }

    fn new(left: i32, bottom: i32, width: i32, height: i32) -> Rect {
        return Rect {
            left,
            bottom,
            width,
            height,
        };
    }
}

struct Circle {
    x: i32,
    y: i32,
    radius: i32,
}

impl Circle {
    fn new(x: i32, y: i32, radius: i32) -> Circle {
        return Circle {
            x,
            y,
            radius,
        };
    }

    fn is_in_rect(&self, rect: &Rect) -> bool {
        Circle::is_in(rect.left, rect.left + rect.width, self.x, self.radius)
            && Circle::is_in(rect.bottom, rect.bottom + rect.height, self.y, self.radius)
    }

    fn is_in(range_min: i32, range_max: i32, center: i32, offset: i32) -> bool {
        range_min <= center - offset && center + offset <= range_max
    }
}

fn answer(line: &str) {
    let numbers: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
    let w = numbers[0];
    let h = numbers[1];
    let x = numbers[2];
    let y = numbers[3];
    let r = numbers[4];

    let rect = Rect::new_from_origin(w, h);
    let circle = Circle::new(x, y, r);
    let output = match circle.is_in_rect(&rect) {
        true => "Yes",
        false => "No"
    };
    println!("{}", output);
}