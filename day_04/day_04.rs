fn data_to_matrix(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect()).collect()
}

fn check_adjacent(x: usize, y: usize, matrix: &Vec<Vec<char>>)-> bool{
    let mut count = 0;
    let directions = [(-1, -1), (-1, 0), (-1, 1),( 0, -1),( 0, 1),( 1, -1), ( 1, 0), ( 1, 1)];

    for (dx, dy) in directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        // bounds check
        if nx >= 0 && ny >= 0 && (nx as usize) < matrix.len() && (ny as usize) < matrix[0].len(){
            if matrix[nx as usize][ny as usize] == '@' {
                count += 1;

                // Abort if more than 4 neighbors are '@'
                if count >= 4 {
                    return false;
                }
            }
        }
    }
    return true;
    
}

fn main() {
    let mut answer = 0;
    let data = "";
    // let matrix_first_puzzle = data_to_matrix(data);
    let mut matrix_second_puzzle = data_to_matrix(data);

    /* First puzzle
    for x in 0..matrix_first_puzzle.len() {
        for y in 0..matrix_first_puzzle[0].len() {
            if matrix_first_puzzle[x][y] == '@' && check_adjacent(x, y, &matrix_first_puzzle) {
                answer += 1;
            }
            
        }
    }
        */

    // Second puzzle
    loop {
        let mut to_remove = vec![];

        for x in 0..matrix_second_puzzle.len() {
            for y in 0..matrix_second_puzzle[0].len() {
                if matrix_second_puzzle[x][y] == '@' && check_adjacent(x, y, &matrix_second_puzzle) {
                    answer += 1;
                    to_remove.push((x, y));
                }
            }
        }

        // If empty we are done break
        if to_remove.is_empty() {
            break;
        }

        for (x, y) in to_remove {
            matrix_second_puzzle[x][y] = 'x';
        }
    }
    println!("{}", answer)

}