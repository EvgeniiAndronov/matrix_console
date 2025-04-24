use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::prelude::*;

const ROWS: usize = 15;
const COLS: usize = 35;

pub fn matr() {
    let matrix = Arc::new(Mutex::new(vec![vec![' '; COLS]; ROWS]));

    for col in 0..COLS {
        let matrix_ref = Arc::clone(&matrix);
        thread::spawn(move || {
            let chars = ['a', 'b', 'c', 'd', 'e', ' ', ' ', ' ', '/', '\\', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            let mut rng = rand::thread_rng();

            loop {
                let new_char = *chars.choose(&mut rng).unwrap();
                {
                    let mut mat = matrix_ref.lock().unwrap();
                    for row in (1..ROWS).rev() {
                        mat[row][col] = mat[row-1][col];
                    }
                    mat[0][col] = new_char;
                }
                let dely_ms = rng.gen_range(250..450);
                thread::sleep(Duration::from_millis(dely_ms));
            }
        });
    }


    loop {
        print!("\x1B[2J\x1B[H");
        {
            let mat = matrix.lock().unwrap();
            for row in 0..ROWS {
                for col in 0..COLS {
                    print!("{} ", mat[row][col]);
                }
                println!();
            }
        }

        thread::sleep(Duration::from_millis(50));
    }
}