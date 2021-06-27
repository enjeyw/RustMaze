use rand::rngs::ThreadRng;
use std::{thread, time, fmt};
use std::io::{Stdout, Write, stdout};
use crossterm::{QueueableCommand, cursor};
use rand::Rng;
use rand::{seq::IteratorRandom, thread_rng};

trait AsChar {
    fn as_char(&self) -> &'static str;
}

#[derive(Copy, Clone)]
enum Point {
    Blank,
    Path,
    Wall,
}

impl AsChar for Point {
    fn as_char(&self) -> &'static str {
        match self {
            Point::Blank => " ",
            Point::Path =>  ".",
            Point::Wall => "X",
        }
    }    
}

struct Matrix {
    vals: Vec<Vec<Point>>,
}

impl Matrix {

    fn new(rows: usize, cols: usize) -> Matrix {
        let mut vec = Vec::new();
        for _ in 0..rows {
            vec.push(vec![Point::Blank; cols]);
        }
        Matrix{vals: vec}
    }

    fn shape(&self) -> (usize, usize) {
        let rows = self.vals.len();
        if rows == 0 {
            (0, 0)
        } else {
            let cols = self.vals[0].len();
            (rows, cols)
        }
    }

    fn at(&self, row: usize, col: usize) -> Point {
        self.vals[row][col]
    }

    fn set(&mut self, row: usize, col: usize, val: Point) {
        self.vals[row][col] = val
    }

    fn render(&self, stdout: &mut Stdout) {
        for row in &self.vals {
            let mut s = String::from("");
            for col in row {
                s.push_str(col.as_char());
            }
            s.push_str("\n");
            stdout.write(s.as_bytes()).ok();
        }
    }

    fn add_row(&mut self) {
        let (_, cols) =  self.shape();
        println!("{}", cols);

        self.vals.push(vec![Point::Blank; cols]);
    }

}

fn generate_path(rows: usize, cols: usize, starting_col: usize) -> Matrix {
    let mut stdout = stdout();

    let mut m = Matrix::new(rows, cols);

    let mut cur_row = 0 as usize;
    let mut cur_col = starting_col;

    let mut rng = thread_rng();

    for _ in 0..10 {
        let (next_row, next_col) = fill_next_point(&mut m, cur_row, cur_col, &mut rng);

        cur_row = next_row;
        cur_col = next_col;
        // println!("Next row is {}, col is {}", cur_row, cur_col);
    }

    return m

}

fn fill_next_point(m: &mut Matrix, cur_row: usize, cur_col: usize, rng: &mut ThreadRng) -> (usize, usize) {

    let (cur_row, cur_col) = (cur_row as i32, cur_col as i32);

    let cardinal_points = vec![
        (cur_row - 1, cur_col), //North
        (cur_row + 1, cur_col), //South
        (cur_row, cur_col - 1), //West
        (cur_row, cur_col + 1)  //East
    ];

    let valid_points = cardinal_points.into_iter().filter(|&p| is_valid_point(&m, p));

    // println!("total vp {}", &valid_points.count());
    
    match valid_points.choose(rng) {
        Some(point) => {
            // println!("Prev row is {}, col is {}", cur_row, cur_col);
            let (new_row, new_col) = point;
            m.set(new_row as usize, new_col as usize, Point::Path);
            (new_row as usize, new_col as usize)
        },
        None => {
            println!("Oh no!");
            (cur_row as usize, cur_col as usize)
        } 
    }

}

fn is_valid_point(m: &Matrix, point: (i32, i32)) -> bool {
    let (total_rows, total_cols) = m.shape();
    let (total_rows, total_cols) = (total_rows as i32, total_cols as i32);

    match point {
        (row, _) if row > total_rows - 1 => false,
        (_, col) if col > total_cols - 1 => false,
        (row, _) if row < 0 => false,
        (_, col) if col < 0 => false,
        (row, col) if matches!(m.at(row as usize, col as usize),Point::Blank) => true,
        (_, _) => false
    }
}

fn main() {
    // let mut m = Matrix::new(2, 5);
    // m.set(0, 4, Point::Path);

    let mut m = generate_path(10,10,5);

    let mut stdout = stdout();
    stdout.queue(cursor::SavePosition).ok();
    m.render(&mut stdout);
    stdout.queue(cursor::RestorePosition).ok();
    stdout.flush().ok();
    // thread::sleep(time::Duration::from_millis(5000));

    // for i in 1..10 {
    //     stdout.queue(cursor::SavePosition).ok();

    //     // let mut vec = Vec::new();
    //     // for j in 1..10 {
    //     //     let mut s = String::from("");

    //     //     for k in 1..10 {
    //     //         let mut rng = rand::thread_rng();
    //     //         let angle: &str = if rng.gen::<bool>() {"/"} else {"\\"};
    //     //         s.push_str(angle);
    //     //     }
    //     //     // vec.push(format!("Here!!! {}  \n", i));
    //     //     s.push_str("\n");
    //     //     vec.push(s);
    //     // }

    //     // for v in &vec {
    //     //     stdout.write(v.as_bytes()).ok();
    //     // }
    //     stdout.queue(cursor::RestorePosition).ok();
    //     stdout.flush().ok();
    //     thread::sleep(time::Duration::from_millis(500));
    // }

    // println!("All Done!");
}