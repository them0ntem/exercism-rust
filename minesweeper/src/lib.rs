use std::char;

const OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn is_mine(board: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    x < board.len() && y < board[0].len() && board[x][y] == '*'
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    println!("{:?}", minefield);

    let minefield: Vec<Vec<char>> = minefield
        .iter()
        .map(|l| l.chars().collect())
        .collect();

    let mut result: Vec<String> = Vec::with_capacity(minefield.len());

    for i in 0..minefield.len() {
        let mut row = String::from("");
        for j in 0..minefield[0].len() {
            if is_mine(&minefield, i, j) {
                row.push('*');
                continue;
            }

            let count = OFFSETS.iter()
                .filter(|offset| is_mine(&minefield, (i as i8 + offset.0) as usize, (j as i8 + offset.1) as usize)).count();

            if count == 0 {
                row.push(' ')
            } else {
                row.push(char::from_digit(count as u32, 10).unwrap())
            }
        }

        result.push(row);
    }

    result
}
