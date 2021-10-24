//! ITP1_6_Cの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_6_C](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_6_C)

use std::collections::HashMap;
use std::str::FromStr;

/// ITP1_6_Cの回答
#[allow(dead_code)]
pub fn main() {
    loop {
        let mut buildings = create_buildings();
        if let Some(notices) = read_inputs() {
            for notice in notices {
                update_buildings(&mut buildings, notice);
            }

            for b in 1..=4 {
                if b > 1 {
                    println!("{}", "#".repeat(20));
                }
                let building = buildings.get(&b).unwrap();
                for f in 1..=3 {
                    let floor = building.floors.get(&f).unwrap();
                    for r in 1..=10 {
                        match floor.get(&r) {
                            None => print!(" 0"),
                            Some(v) => print!(" {}", v)
                        };
                    }
                    println!();
                }
            }

            continue;
        }
        return;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Building {
    floors: HashMap<i32, HashMap<i32, i32>>,
}

fn create_buildings() -> HashMap<i32, Building> {
    let mut result = HashMap::new();
    for building_number in 1..=4 {
        let mut floors = HashMap::new();
        for f in 1..=3 {
            floors.insert(f, HashMap::new());
        }
        result.insert(building_number, Building { floors });
    }
    result
}

#[derive(Debug, Eq, PartialEq)]
struct Notice {
    b: i32,
    f: i32,
    r: i32,
    v: i32,
}

impl Notice {
    fn new(b: i32, f: i32, r: i32, v: i32) -> Notice {
        Notice { b, f, r, v }
    }
}

impl FromStr for Notice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(());
        }
        let numbers: Vec<i32> = s
            .split(' ')
            .map(|x| x.parse::<i32>()
                .unwrap())
            .collect();
        Ok(Notice::new(numbers[0], numbers[1], numbers[2], numbers[3]))
    }
}

fn update_buildings(buildings: &mut HashMap<i32, Building>, notice: Notice) {
    let building = buildings.get_mut(&notice.b).unwrap();
    let rooms = building.floors.get_mut(&notice.f).unwrap();
    *rooms.entry(notice.r).or_insert(0) += notice.v;
}

fn read_inputs() -> Option<Vec<Notice>> {
    if let Some(line) = read_line() {
        let mut result = Vec::new();

        let input_count = line.trim().parse::<i32>().unwrap();
        for _ in 0..input_count {
            if let Some(line) = read_line() {
                result.push(Notice::from_str(&line).unwrap());
            } else {
                return None;
            }
        }
        return Some(result);
    }
    return None;
}

fn read_line() -> Option<String> {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(line.to_string())
            }
        }
        Err(_) => None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_buildings() {
        let buildings = create_buildings();
        assert_eq!(4, buildings.len());

        for building_number in 1..=4 {
            let building = buildings.get(&building_number);
            assert_ne!(None, building);
            let building = building.unwrap();
            assert_eq!(3, building.floors.len());

            for floor_number in 1..=3 {
                let floor = building.floors.get(&floor_number);
                assert_ne!(None, floor);
            }
        }
    }

    #[test]
    fn test_notice_from_str() {
        assert_eq!(Ok(Notice::new(1, 1, 1, 1)), Notice::from_str("1 1 1 1"));
        assert_eq!(Ok(Notice::new(2, 3, 4, 5)), Notice::from_str("2 3 4 5\n"));
    }

    #[test]
    fn test_update_buildings() {
        let mut buildings = create_buildings();
        update_buildings(&mut buildings, Notice::new(1, 1, 1, 1));
        assert_eq!(1, *buildings.get(&1).unwrap().floors.get(&1).unwrap().get(&1).unwrap());

        update_buildings(&mut buildings, Notice::new(2, 2, 2, 2));
        assert_eq!(2, *buildings.get(&2).unwrap().floors.get(&2).unwrap().get(&2).unwrap());

        update_buildings(&mut buildings, Notice::new(1, 1, 1, -1));
        assert_eq!(0, *buildings.get(&1).unwrap().floors.get(&1).unwrap().get(&1).unwrap());
    }
}