//! ITP1_8_Cの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_8_C](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_8_C)

use std::collections::{HashMap};
use std::iter::{FromIterator};

#[allow(dead_code)]
pub fn main() {
    let mut counter = CharacterCounter::new();
    loop {
        let mut line = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut line) {
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        line.chars().for_each(|x| counter.increment_char(x));

        continue;
    }

    for (c, n) in counter.char_counts() {
        println!("{} : {}", c, n);
    }
}

/// 文字数カウンター。
#[derive(Debug)]
struct CharacterCounter {
    char_count_map: HashMap<char, i32>,
}

impl CharacterCounter {
    fn new() -> Self {
        CharacterCounter {
            char_count_map: HashMap::from_iter(Self::all_char_range())
        }
    }

    // AIZU ONLINE JUDGE では、Range.mapがコンパイルできないのでわざわざVecを返す😔
    fn all_char_range() -> Vec<(char, i32)> {
        let mut result = Vec::new();
        for i in 0..26 {
            let c = (('a' as u8) + (i as u8)) as char;
            result.push((c, 0));
        }
        result
    }

    fn char_counts(&self) -> Vec<(char, i32)> {
        let mut result: Vec<(char, i32)> = self.char_count_map.iter()
            .map(|(c, n)| (*c, *n))
            .collect();
        result.sort_by(|(c1, _), (c2, _)| (*c1).cmp(c2));
        result
    }

    fn increment_char(&mut self, c: char) {
        let c = c.to_ascii_lowercase();
        if !self.char_count_map.contains_key(&c) {
            return;
        }
        *self.char_count_map.entry(c).or_insert(0) += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_character_counter_new() {
        // 初期状態ではどの文字も0個。
        let expected_char_counts: Vec<(char, i32)> = ('a'..='z').map(|x| (x, 0)).collect();
        assert_eq!(expected_char_counts, CharacterCounter::new().char_counts());
    }

    #[test]
    fn test_increment_char() {
        let mut target = CharacterCounter::new();
        let chars = vec!['T', 'h', 'i', '.'];
        let expected_chars: Vec<char> = chars.iter().map(|x| (*x).to_ascii_lowercase()).collect();
        chars.iter().for_each(|c| {
            target.increment_char(*c);
        });

        let actual = target.char_counts();
        assert_eq!(26, actual.len());
        for (c, n) in actual {
            if expected_chars.contains(&c) {
                assert_eq!(1, n, "{}", c)
            } else {
                assert_eq!(0, n, "{}", c)
            }
        }
    }
}