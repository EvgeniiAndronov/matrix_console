use std::thread::sleep;
use rand::prelude::SliceRandom;

const COLOMN: usize = 40;
const ROWS: usize = 15;

fn sleeps(sec: u64) {
    sleep(std::time::Duration::from_secs(sec)/2);
    print!("\x1B[2J\x1B[H");
}

fn take_previous(matrix: &mut Vec<Vec<char>>) {
    let chars: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', ' ', ' ', ' ', ' ', ' ', ' ', ' '];

    for i in (1..ROWS).rev() {
        for j in 0..COLOMN {
            matrix[i][j] = matrix[i-1][j];
        }
    }

    for i in 0..COLOMN {
        if let Some(&random_char) = chars.choose(&mut rand::thread_rng()) {
            matrix[0][i] = random_char;
        }
    }
}

pub fn start_matrix_sync() {
    let mut matrix = vec![vec!['0'; COLOMN]; ROWS];

    for i in 0..25 {
        take_previous(&mut matrix);
        for line in &matrix {
            let line_str: String = line.iter().collect();
            println!("{}", line_str);
        }
        sleeps(1);
    }
}
