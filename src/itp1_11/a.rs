//! ITP1_11_Aの回答
//! [https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/11/ITP1_11_A](https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/11/ITP1_11_A)

use std::io::BufRead;

#[allow(dead_code)]
pub fn main() {
    let dataset = Dataset::read(std::io::stdin().lock());
    let mut dice = Dice::new();
    dataset.roll_dice_by_directions(&mut dice);
    let result = dataset.get_number_at_dice_position(dice.center);
    println!("{}", result);
}

/// 動かす方向。
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    E,
    N,
    S,
    W,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'E' => Direction::E,
            'N' => Direction::N,
            'S' => Direction::S,
            'W' => Direction::W,
            _ => panic!("invalid direction")
        }
    }
}

/// サイコロの1〜6に対応する数値と、転がす方向列。
#[derive(Debug, Eq, PartialEq)]
struct Dataset {
    numbers: Vec<i32>,
    directions: Vec<Direction>,
}

impl Dataset {
    fn read<T: BufRead>(mut reader: T) -> Self {
        Dataset {
            numbers: read_numbers(&mut reader),
            directions: read_directions(&mut reader),
        }
    }

    fn roll_dice_by_directions(&self, dice: &mut Dice) {
        self.directions.iter().for_each(|direction| {
            dice.roll(*direction);
        })
    }

    /// ダイスの位置に相当する数値を取得する。
    fn get_number_at_dice_position(&self, dice_position: i32) -> i32 {
        let index = (dice_position - 1) as usize;
        self.numbers[index]
    }
}

/// サイコロ。
/// このサイコロは以下の展開図から成る。
///
///   1
/// 4 2 3 5
///   6
///
/// デフォルトのポジションは1の綿が上に見える場合とする。
///   5
/// 4 1 3 6
///   2
#[derive(Debug, Eq, PartialEq)]
struct Dice {
    top: i32,
    left: i32,
    center: i32,
    right: i32,
    right_right: i32,
    bottom: i32,
}

impl Dice {
    /// 新しい`Dice`インスタンスを作成する。
    fn new() -> Self {
        Dice {
            top: 5,
            left: 4,
            center: 1,
            right: 3,
            right_right: 6,
            bottom: 2,
        }
    }

    /// ダイスを`direction`の方向に一面転がす。
    fn roll(&mut self, direction: Direction) {
        match direction {
            Direction::E => {
                let left = self.right_right;
                self.right_right = self.right;
                self.right = self.center;
                self.center = self.left;
                self.left = left;
            }
            Direction::N => {
                let bottom = self.right_right;
                self.right_right = self.top;
                self.top = self.center;
                self.center = self.bottom;
                self.bottom = bottom;
            }
            Direction::S => {
                let top = self.right_right;
                self.right_right = self.bottom;
                self.bottom = self.center;
                self.center = self.top;
                self.top = top;
            }
            Direction::W => {
                let right = self.right_right;
                self.right_right = self.left;
                self.left = self.center;
                self.center = self.right;
                self.right = right;
            }
        }
    }
}

fn read_line<T: BufRead>(mut reader: T) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().to_owned()
}

fn read_numbers<T: BufRead>(reader: T) -> Vec<i32> {
    read_line(reader)
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_directions<T: BufRead>(reader: T) -> Vec<Direction> {
    read_line(reader)
        .chars()
        .map(|x| x.into())
        .collect()
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_new_dice() {
        assert_eq!(Dice {
            top: 5,
            left: 4,
            center: 1,
            right: 3,
            right_right: 6,
            bottom: 2,
        }, Dice::new());
    }

    #[test]
    fn test_roll_dice_1() {
        let mut dice = Dice::new();
        dice.roll(Direction::S);
        assert_eq!(Dice {
            top: 6,
            left: 4,
            center: 5,
            right: 3,
            right_right: 2,
            bottom: 1,
        }, dice);
        dice.roll(Direction::E);
        assert_eq!(Dice {
            top: 6,
            left: 2,
            center: 4,
            right: 5,
            right_right: 3,
            bottom: 1,
        }, dice);
    }

    #[test]
    fn test_roll_dice_2() {
        let mut dice = Dice::new();
        dice.roll(Direction::E);
        assert_eq!(Dice {
            top: 5,
            left: 6,
            center: 4,
            right: 1,
            right_right: 3,
            bottom: 2,
        }, dice);
        dice.roll(Direction::E);
        assert_eq!(Dice {
            top: 5,
            left: 3,
            center: 6,
            right: 4,
            right_right: 1,
            bottom: 2,
        }, dice);
        dice.roll(Direction::S);
        dice.roll(Direction::W);
        dice.roll(Direction::N);

        assert_eq!(Dice {
            top: 4,
            left: 5,
            center: 6,
            right: 2,
            right_right: 1,
            bottom: 3,
        }, dice);
    }

    #[test]
    fn test_read_line() {
        assert_eq!("abc", &read_line(Cursor::new("abc\n")));
    }

    #[test]
    fn test_read_numbers() {
        assert_eq!(vec![1, 2, 4, 8, 16, 32], read_numbers(Cursor::new("1 2 4 8 16 32\n")));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_read_directions() {
        assert_eq!(vec![Direction::E, Direction::E, Direction::S, Direction::W, Direction::N],
                   read_directions(Cursor::new("EESWN\n")));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_read_dataset() {
        assert_eq!(
            Dataset {
                numbers: vec![1, 2, 4, 8, 16, 32],
                directions: vec![Direction::E, Direction::E, Direction::S, Direction::W, Direction::N],
            },
            Dataset::read(Cursor::new("1 2 4 8 16 32\nEESWN\n")));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_dataset_roll_dice() {
        let dataset = Dataset {
            numbers: vec![1, 2, 4, 8, 16, 32],
            directions: vec![Direction::E, Direction::E, Direction::S, Direction::W, Direction::N],
        };

        let mut dice = Dice::new();
        dataset.roll_dice_by_directions(&mut dice);
        assert_eq!(6, dice.center);
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_dataset_get_number_at_dice_position() {
        let dataset = Dataset {
            numbers: vec![1, 2, 4, 8, 16, 32],
            directions: vec![Direction::E, Direction::E, Direction::S, Direction::W, Direction::N],
        };

        let mut dice = Dice::new();
        dataset.roll_dice_by_directions(&mut dice);
        assert_eq!(32, dataset.get_number_at_dice_position(dice.center));
    }
}