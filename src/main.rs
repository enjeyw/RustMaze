use rand::rngs::ThreadRng;
use std::{thread, time};
use std::io::{ Write, stdout};
use crossterm::{QueueableCommand, cursor};
use rand::{ seq::IteratorRandom, thread_rng};

mod matrix;
use matrix::{Point, PointDelta, MazeObj, Matrix};

fn generate_path(rows: usize, cols: usize, starting_col: usize) -> Matrix<MazeObj> {

    let mut m = Matrix::new(rows, cols);

    let mut cur_point = Point{x: 0, y: starting_col};

    let mut rng = thread_rng();

    for _ in 0..10 {
        let next_point = fill_next_point(&mut m, cur_point, &mut rng);

        fill_walls(&mut m, cur_point);

        cur_point = next_point;
    }

    return m

}

fn fill_next_point(m: &mut Matrix<MazeObj>, cur_point: Point, rng: &mut ThreadRng) -> Point {

    let cardinal_points = vec![
        cur_point.translate(PointDelta{dx: -1, dy:  0}), //North
        cur_point.translate(PointDelta{dx:  1, dy:  0}), //South
        cur_point.translate(PointDelta{dx:  0, dy: -1}), //West
        cur_point.translate(PointDelta{dx:  0, dy:  1}), //East
    ];

    let valid_points = cardinal_points
        .into_iter()
        .filter(|op| match op {
            Some(p) => is_valid_point(m, p),
            None => false
        }).flatten();

    
    match valid_points.choose(rng) {
        Some(point) => {
            // println!("Prev row is {}, col is {}", cur_row, cur_col);
            m.set(point, MazeObj::Path);
            point
        },
        None => {
            println!("Oh no!");
            cur_point
        } 
    }

}

fn fill_walls(m: &mut Matrix<MazeObj>, point: Point) {

    let surrounding_points = vec![
        point.translate(PointDelta{dx: -1, dy:  0}), //North
        point.translate(PointDelta{dx:  1, dy:  0}), //South
        point.translate(PointDelta{dx:  0, dy: -1}), //West
        point.translate(PointDelta{dx:  0, dy:  1}), //East

        point.translate(PointDelta{dx: -1, dy: -1}), //North West
        point.translate(PointDelta{dx: -1, dy:  1}), //North East
        point.translate(PointDelta{dx:  1, dy: -1}), //South West
        point.translate(PointDelta{dx:  1, dy:  1}), //South East

        // (row - 1, col), //North
        // (row + 1, col), //South
        // (row, col - 1), //West
        // (row, col + 1),  //East
        // (row - 1, col - 1), //North West
        // (row - 1, col + 1), //North East
        // (row + 1, col - 1), //South West
        // (row + 1, col + 1), //South East
    ];

    for sp in surrounding_points {
        if let Some(p) = sp {
            if is_valid_point(&m, &p) {
                m.set(p, MazeObj::Wall);
            }
        }
    }

}

fn is_valid_point(m: &Matrix<MazeObj>, point: &Point) -> bool {
    let (total_rows, total_cols) = m.shape();

    match point {
        Point{x, y} if (
            x <= &(total_rows - 1) && y <= &(total_cols - 1) && matches!(m.at(point), MazeObj::Blank)
        ) => true,
        _ => false
    }
}

fn main() {
    // let mut m = Matrix::new(2, 5);
    // m.set(0, 4, Point::Path);

    let m = generate_path(10,10,5);
    let mut stdout = stdout();
    stdout.queue(cursor::SavePosition).ok();
    m.render(&mut stdout);
    stdout.queue(cursor::RestorePosition).ok();
    stdout.flush().ok();
    thread::sleep(time::Duration::from_millis(5000));

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