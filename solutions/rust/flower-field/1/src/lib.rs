pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    if rows == 0 {
        return vec![];
    }
    let cols = garden[0].len();

    let board: Vec<Vec<char>> = garden.iter().map(|&r| r.chars().collect()).collect();

    let mut result = vec![vec![' '; cols]; rows];


    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];


    for row in 0..rows {
        for col in 0..cols {
            if board[row][col] == '*' {
                result[row][col] = '*'; 
            } else {
    
                let mut count = 0;
                for &(dr, dc) in &directions {
                    let new_row = row as isize + dr;
                    let new_col = col as isize + dc;
                    if new_row >= 0
                        && new_row < rows as isize
                        && new_col >= 0
                        && new_col < cols as isize
                        && board[new_row as usize][new_col as usize] == '*'
                    {
                        count += 1;
                    }
                }
                if count > 0 {
                    result[row][col] = std::char::from_digit(count as u32, 10).unwrap();
                }
            }
        }
    }

    result.into_iter().map(|r| r.into_iter().collect()).collect()
}
