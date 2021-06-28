use std::io::{Stdout, Write, stdout};

pub struct Matrix<T> {
    vals: Vec<Vec<T>>,
}

trait MatrixValue {
    fn default_val() -> Self;
    fn as_char(&self) -> &'static str;
}

impl<T> Matrix<T>
where T: MatrixValue + Copy
{
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
        let mut vec = Vec::new();
        for _ in 0..rows {
            vec.push(vec![T::default_val(); cols]);
        }
        Matrix{vals: vec}
    }

    pub fn shape(&self) -> (usize, usize) {
        let rows = self.vals.len();
        if rows == 0 {
            (0, 0)
        } else {
            let cols = self.vals[0].len();
            (rows, cols)
        }
    }

    pub fn at(&self, row: usize, col: usize) -> T {
        self.vals[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        self.vals[row][col] = val
    }

    pub fn render(&self, stdout: &mut Stdout) {
        for row in &self.vals {
            let mut s = String::from("");
            for col in row {
                s.push_str(col.as_char());
            }
            s.push_str("\n");
            stdout.write(s.as_bytes()).ok();
        }
    }

    pub fn add_row(&mut self) {
        let (_, cols) =  self.shape();
        println!("{}", cols);

        self.vals.push(vec![T::default_val(); cols]);
    }

}

#[derive(Copy, Clone)]
pub enum Point {
    Blank,
    Path,
    Wall,
}

impl MatrixValue for Point {
    fn default_val() -> Point {
        Point::Blank
    }

    fn as_char(&self) -> &'static str {
        match self {
            Point::Blank => " ",
            Point::Path =>  ".",
            Point::Wall => "X",
        }
    }  
}