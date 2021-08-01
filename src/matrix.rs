use std::io::{Stdout, Write};

#[derive(Debug,  Copy, Clone, Hash, Eq, PartialEq,)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct PointDelta {
    pub dx: isize,
    pub dy: isize
}



impl Point {

    pub fn translate(self, delta: PointDelta) -> Option<Point> {
        let PointDelta{dx, dy} = delta;
        let Point{x, y} = self;

        match ((x as isize).checked_add(dx), (y as isize).checked_add(dy)) {
            (Some(new_x), Some(new_y)) if new_x >= 0 && new_y >= 0 => Some(Point{x: new_x as usize, y: new_y as usize}),
            (_ , _) => None
        }
    }

    // pub fn add(self, other: Point) -> Option<Point> {
    //     Point::checked_math(usize::checked_add, self, other)
    // }   

    // pub fn sub(self, other: Point) -> Option<Point> {
    //     Point::checked_math(usize::checked_sub, self, other)
    // }

    // fn checked_math(f: fn(usize, usize) -> Option<usize>, s: Point, o: Point) ->  Option<Point> {
    //     match (f(s.x, o.x) , f(s.y, o.y)) {
    //         (Some(x), Some(y)) => Some(Point{x, y}),
    //         (_ , _) => None
    //     }
    // }

    // pub fn sub(self, other: Point) -> Option<Point> {
    //     match (self.x.checked_sub(other.x) , self.y.checked_sub(other.y)) {
    //         (Some(x), Some(y)) => Some(Point{x, y}),
    //         (_ , _) => None
    //     }
    // }

    
}

pub struct Matrix<T> {
    vals: Vec<Vec<T>>,
}

pub trait MatrixValue {
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

    pub fn at(&self, p: &Point) -> T {
        self.vals[p.x][p.y]
    }

    pub fn set(&mut self, p: Point, val: T) {
        self.vals[p.x][p.y] = val
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

    // pub fn add_row(&mut self) {
    //     let (_, cols) =  self.shape();
    //     println!("{}", cols);

    //     self.vals.push(vec![T::default_val(); cols]);
    // }

}

#[derive(Copy, Clone)]
pub enum MazeObj {
    Blank,
    Path,
    Wall,
}

impl MatrixValue for MazeObj {
    fn default_val() -> MazeObj {
        MazeObj::Blank
    }

    fn as_char(&self) -> &'static str {
        match self {
            MazeObj::Blank => "#",
            MazeObj::Path =>  ".",
            MazeObj::Wall => "X",
        }
    }  
}