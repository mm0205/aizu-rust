use std::io::BufRead;
use std::iter::FromIterator;
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some((mat, vec)) = read_input(std::io::stdin().lock()) {
            let result = mat.mul(&vec);
            result.items.iter().for_each(|x| {
                println!("{}", x)
            });
            continue;
        }
        return;
    }
}

trait MyNum: Copy {
    fn zero() -> Self;
    fn add(&self, a: Self) -> Self;
    fn mul(&self, a: Self) -> Self;
}

#[derive(Debug, Eq, PartialEq)]
struct Vector<T> {
    items: Vec<T>,
}

impl<T> Vector<T>
    where
        T: MyNum {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn dot_product(self: &Self, other: &Self) -> T {
        if self.len() != other.len() {
            panic!("サイズが違うベクトルの内積は計算できません😔")
        }
        let mut result: T = T::zero();
        for (a, b) in self.items.iter().zip(other.items.iter()) {
            result = result.add(a.mul(*b));
        }
        result
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T> FromIterator<T> for Vector<T>
    where T: MyNum {
    fn from_iter<U: IntoIterator<Item=T>>(iter: U) -> Self {
        let mut result = Vector::<T>::new();
        for x in iter {
            result.items.push(x);
        }
        result
    }
}

impl FromStr for Vector<i32> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        for field in s.trim().split(' ') {
            match field.parse::<i32>() {
                Ok(n) => v.push(n),
                _ => return Err(()),
            };
        }
        Ok(Self::from_iter(v))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Matrix<T> where T: MyNum {
    rows: Vec<Vector<T>>,
}

impl<T> Matrix<T> where T: MyNum {
    fn new() -> Self {
        Matrix { rows: Vec::new() }
    }

    fn mul(&self, v: &Vector<T>) -> Vector<T> {
        let mut result = Vector::from_iter(vec![T::zero(); self.rows.len()]);

        for i in 0..self.rows.len() {
            let result_i = self.rows[i].dot_product(v);
            result.items[i] = result_i;
        }
        result
    }
}

impl Matrix<i32> {
    fn read_matrix<T: BufRead>(n: i32, mut reader: T) -> Option<Self> {
        let mut rows = Vec::new();
        for _ in 0..n {
            if let Some(numbers) = read_numbers(&mut reader) {
                let v = Vector::from_iter(numbers);
                rows.push(v);
                continue;
            }
            return None;
        }
        Some(Matrix::from_iter(rows))
    }
}

impl<T> FromIterator<Vector<T>> for Matrix<T> where T: MyNum {
    fn from_iter<U: IntoIterator<Item=Vector<T>>>(iter: U) -> Self {
        let mut result = Matrix::new();
        for row in iter {
            result.rows.push(row);
        }
        result
    }
}

impl MyNum for i32 {
    fn zero() -> Self {
        0
    }

    fn add(&self, a: Self) -> Self {
        *self + a
    }

    fn mul(&self, a: Self) -> Self {
        *self * a
    }
}


fn read_numbers<T: BufRead>(mut reader: T) -> Option<Vec<i32>> {
    let mut line = String::new();
    if let Ok(_) = reader.read_line(&mut line) {
        let mut result = Vec::new();
        for field in line.trim().split(' ') {
            if let Ok(n) = field.parse() {
                result.push(n);
                continue;
            }
            return None;
        }
        return Some(result);
    }
    None
}

fn read_input<T: BufRead>(mut reader: T) -> Option<(Matrix<i32>, Vector<i32>)> {
    if let Some(numbers) = read_numbers(&mut reader) {
        let n = numbers[0];
        let m = numbers[1];

        if let Some(mat) = Matrix::read_matrix(n, &mut reader) {
            if mat.rows.iter().any(|x| x.len() != m as usize) {
                return None;
            }

            let mut v = Vec::new();
            for _ in 0..m {
                v.push(read_numbers(&mut reader).unwrap()[0]);
            }

            return Some((mat, Vector::from_iter(v)));
        }
    }
    None
}


#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    fn create_test_mat() -> Matrix<i32> {
        Matrix::from_iter(vec![
            Vector::from_iter(vec![1, 2, 0, 1]),
            Vector::from_iter(vec![0, 3, 0, 1]),
            Vector::from_iter(vec![4, 1, 1, 0]),
        ])
    }

    #[test]
    fn test_vector_new() {
        assert_eq!(Vector::<i32> { items: Vec::new() }, Vector::<i32>::new());
    }

    #[test]
    fn test_vector_from_iter() {
        assert_eq!(vec![1, 2, 3], *Vector::from_iter(vec![1, 2, 3]).items);
    }

    #[test]
    fn test_vector_dot_product() {
        let a = Vector::from_iter(vec![1, 2, 3]);
        let b = Vector::from_iter(vec![1, 2, 3]);
        assert_eq!(14, a.dot_product(&b));

        let a = Vector::from_iter(vec![1, 2, 0, 1]);
        let b = Vector::from_iter(vec![1, 2, 3, 0]);
        assert_eq!(5, a.dot_product(&b));
    }

    #[test]
    fn test_matrix_new() {
        assert_eq!(Matrix::<i32> { rows: Vec::new() }, Matrix::<i32>::new());
    }

    #[test]
    fn test_matrix_from_iter() {
        let src = vec![Vector::from_iter(vec![1, 2, 3]), Vector::from_iter(vec![4, 5, 6])];
        let result = Matrix::from_iter(src);
        assert_eq!(Matrix {
            rows: vec![
                Vector {
                    items: vec![1, 2, 3]
                },
                Vector {
                    items: vec![4, 5, 6]
                },
            ]
        }, result);
    }

    #[test]
    fn test_vector_len() {
        assert_eq!(0, Vector::<i32>::new().len());
        assert_eq!(1, Vector::from_iter(vec![1]).len());
        assert_eq!(3, Vector::from_iter(vec![1, 2, 3]).len());
    }

    #[test]
    fn test_matrix_mul() {
        let a = Matrix::from_iter(vec![Vector::from_iter(vec![2]), Vector::from_iter(vec![3])]);
        let x = Vector::from_iter(vec![3]);
        assert_eq!(2, a.mul(&x).items.len());
        assert_eq!(6, a.mul(&x).items[0]);
        assert_eq!(9, a.mul(&x).items[1]);

        let a = create_test_mat();
        let x = Vector::from_iter(vec![1, 2, 3, 0]);
        assert_eq!(3, a.mul(&x).items.len());
        assert_eq!(5, a.mul(&x).items[0]);
        assert_eq!(6, a.mul(&x).items[1]);
        assert_eq!(9, a.mul(&x).items[2]);
    }

    #[test]
    fn test_read_numbers() {
        let input = Cursor::new(b"1 2 3 4 5\n");
        assert_eq!(Some(vec![1, 2, 3, 4, 5]), read_numbers(input));

        let input = Cursor::new(b"\n");
        assert_eq!(None, read_numbers(input));
    }

    #[test]
    fn test_read_matrix() {
        let result = Matrix::from_iter(vec![Vector::from_iter(vec![1, 2, 3]),
                                            Vector::from_iter(vec![4, 5, 6]),
                                            Vector::from_iter(vec![7, 8, 9])]);
        assert_eq!(Some(result), Matrix::read_matrix(3, Cursor::new(b"1 2 3\n4 5 6\n7 8 9\n")));
    }

    #[test]
    fn test_read_input() {
        let input = Cursor::new(b"3 4\n1 2 0 1\n0 3 0 1\n4 1 1 0\n1\n2\n3\n0");

        let ex_mat = create_test_mat();
        let ex_vec = Vector::from_iter(vec![1, 2, 3, 0]);

        let result = read_input(input);
        assert_eq!(Some((ex_mat, ex_vec)), result);
    }
}