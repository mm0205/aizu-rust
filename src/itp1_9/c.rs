//! ITP1_9_C回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_9_C&lang=ja](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_9_C&lang=ja)

use std::cmp::Ordering;
use std::io::BufRead;

#[allow(dead_code)]
pub fn main() {
    let mut game = Game::new();
    game.execute(std::io::stdin().lock());
    println!("{} {}", game.score.taro, game.score.hanako);
}

#[derive(Debug, Eq, PartialEq)]
struct Score {
    taro: i32,
    hanako: i32,
}

impl Score {
    fn new() -> Self {
        Score { taro: 0, hanako: 0 }
    }

    fn update(&mut self, ordering: Ordering) {
        let addition = match ordering {
            Ordering::Less => (0, 3),
            Ordering::Equal => (1, 1),
            Ordering::Greater => (3, 0),
        };
        self.taro = self.taro + addition.0;
        self.hanako = self.hanako + addition.1;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    score: Score,
}

impl Game {
    fn new() -> Self {
        Game { score: Score::new() }
    }

    fn execute<T: BufRead>(&mut self, mut reader: T) {
        let n = self.read_n(&mut reader);
        for _ in 0..n {
            let (taro, hanako) = self.read_cards(&mut reader);
            self.score.update(taro.cmp(&hanako));
        }
    }

    fn read_n<T: BufRead>(&self, mut reader: T) -> i32 {
        self.read_line(&mut reader).parse().unwrap()
    }

    fn read_cards<T: BufRead>(&self, mut reader: T) -> (String, String) {
        let ss: Vec<String> = self.read_line(&mut reader)
            // .split_once(' ') を使えばよいのだが、AIZU ONLINE JUDGEのRustのバージョンにはないみたい。
            .split(' ')
            .map(|x| x.to_owned())
            .collect();
        (ss[0].to_owned(), ss[1].to_owned())
    }

    fn read_line<T: BufRead>(&self, mut reader: T) -> String {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        line.trim().to_owned()
    }
}


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_score_new() {
        assert_eq!(Score { taro: 0, hanako: 0 }, Score::new());
    }

    #[test]
    fn test_update_score_lt() {
        let mut score = Score::new();
        score.update(Ordering::Less);
        assert_eq!(Score { taro: 0, hanako: 3 }, score);
    }

    #[test]
    fn test_update_score_eq() {
        let mut score = Score::new();
        score.update(Ordering::Equal);
        assert_eq!(Score { taro: 1, hanako: 1 }, score);
    }

    #[test]
    fn test_update_score_gt() {
        let mut score = Score::new();
        score.update(Ordering::Greater);
        assert_eq!(Score { taro: 3, hanako: 0 }, score);
    }

    #[test]
    fn test_update_score_multi() {
        let mut score = Score::new();
        score.update(Ordering::Greater);
        score.update(Ordering::Equal);
        assert_eq!(Score { taro: 4, hanako: 1 }, score);
    }

    #[test]
    fn test_game_new() {
        assert_eq!(Game { score: Score { taro: 0, hanako: 0 } }, Game::new());
    }

    #[test]
    fn test_game_read_line() {
        let g = Game::new();
        assert_eq!("cat dog".to_owned(), g.read_line(Cursor::new("cat dog\n")));
    }

    #[test]
    fn test_game_read_n() {
        let g = Game::new();
        assert_eq!(3, g.read_n(Cursor::new("3\n")));
    }

    #[test]
    fn test_game_read_cards() {
        let g = Game::new();
        assert_eq!(("cat".to_owned(), "dog".to_owned()), g.read_cards(Cursor::new("cat dog\n")));
    }

    #[test]
    fn test_game_execute() {
        let mut g = Game::new();
        let input = Cursor::new("3\ncat dog\nfish fish\nlion tiger\n");
        g.execute(input);
        assert_eq!(Score { taro: 1, hanako: 7 }, g.score);
    }
}