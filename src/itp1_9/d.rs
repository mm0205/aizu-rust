//! ITP1_9_Dの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_9_D&lang=ja](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_9_D&lang=ja)

/// ITP1_9_Dの回答
#[allow(dead_code)]
pub fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let computer = Computer::new(line.trim());
    let _ = execute(computer);
}

fn execute(computer: Computer) -> Computer {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let q: i32 = line.trim().parse().unwrap();

    execute_n(computer, q)
}

fn execute_n(computer: Computer, n: i32) -> Computer {
    if n <= 0 {
        computer
    } else {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        execute_n(computer.apply_command(&Command::from(line.trim())), n - 1)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Computer {
    str: String,
}

impl Clone for Computer {
    fn clone(&self) -> Self {
        Self {
            str: self.str.to_owned()
        }
    }
}

impl Computer {
    fn new(str: &str) -> Self {
        Computer { str: str.to_owned() }
    }

    fn apply_command(&self, command: &Command) -> Self {
        match command {
            Command::Print { a, b } => self.print(*a, *b),
            Command::Reverse { a, b } => self.reverse(*a, *b),
            Command::Replace { a, b, p } => self.replace(*a, *b, p)
        }
    }

    fn print(&self, a: usize, b: usize) -> Self {
        println!("{}", self.format_for_print(a, b));
        self.clone()
    }

    fn format_for_print(&self, a: usize, b: usize) -> String {
        format!("{}", &self.str[a..=b])
    }

    fn reverse(&self, a: usize, b: usize) -> Self {
        let s1 = &self.str[0..a];
        let s2 = &self.str[a..=b].chars().rev().collect::<String>();
        let s3 = &self.str[(b + 1)..self.str.len()];
        Self { str: s1.to_owned() + s2.as_str() + s3 }
    }

    fn replace(&self, a: usize, b: usize, p: &str) -> Self {
        let s1 = &self.str[0..a];
        let s3 = &self.str[(b + 1)..self.str.len()];
        Self { str: s1.to_owned() + p + s3 }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Print { a: usize, b: usize },
    Reverse { a: usize, b: usize },
    Replace { a: usize, b: usize, p: String },
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let v: Vec<String> = s.trim()
            .split(' ')
            .map(|x| x.to_owned())
            .collect();

        match v[0].as_str() {
            "print" => Command::Print { a: v[1].parse().unwrap(), b: v[2].parse().unwrap() },
            "reverse" => Command::Reverse { a: v[1].parse().unwrap(), b: v[2].parse().unwrap() },
            "replace" => Command::Replace { a: v[1].parse().unwrap(), b: v[2].parse().unwrap(), p: v[3].to_owned() },
            _ => panic!()
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    // noinspection SpellCheckingInspection
    #[test]
    fn test_computer_new() {
        assert_eq!(Computer { str: "abcde".to_owned() }, Computer::new("abcde"));
    }

    #[test]
    fn test_command_from() {
        assert_eq!(Command::Replace { a: 1, b: 3, p: "xyz".to_owned() }, Command::from("replace 1 3 xyz\n"));
        assert_eq!(Command::Reverse { a: 0, b: 2 }, Command::from("reverse 0 2\n"));
        assert_eq!(Command::Print { a: 1, b: 4 }, Command::from("print 1 4\n"));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_computer_print() {
        let target = Computer::new("abcde");
        assert_eq!("a", target.format_for_print(0, 0));
        assert_eq!("abc", target.format_for_print(0, 2));
        assert_eq!("bcd", target.format_for_print(1, 3));
        assert_eq!("cde", target.format_for_print(2, 4));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_computer_reverse() {
        assert_eq!("cbade", Computer::new("abcde").reverse(0, 2).str);
        assert_eq!("adcbe", Computer::new("abcde").reverse(1, 3).str);
        assert_eq!("abedc", Computer::new("abcde").reverse(2, 4).str);
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_computer_replace() {
        assert_eq!("xyzde", Computer::new("abcde").replace(0, 2, "xyz").str);
        assert_eq!("axyze", Computer::new("abcde").replace(1, 3, "xyz").str);
        assert_eq!("abxyz", Computer::new("abcde").replace(2, 4, "xyz").str);
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_computer_apply_command_1() {
        let target = Computer::new("abcde");

        let target = target.apply_command(&Command::Replace { a: 1, b: 3, p: "xyz".to_owned() });
        assert_eq!("axyze".to_owned(), target.str);

        let target = target.apply_command(&Command::Reverse { a: 0, b: 2 });
        assert_eq!("yxaze".to_owned(), target.str);

        assert_eq!("xaze".to_owned(), target.format_for_print(1, 4));
    }

    #[test]
    fn test_computer_apply_command_2() {
        let target = Computer::new("xyz");
        let before = target.str.clone();
        assert_eq!(before, target.apply_command(&Command::Print { a: 0, b: 2 }).str);
        assert_eq!("abc".to_owned(), target.apply_command(&Command::Replace { a: 0, b: 2, p: "abc".to_owned() }).str);
    }
}