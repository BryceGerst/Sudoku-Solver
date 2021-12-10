use rand::seq::SliceRandom;
use rand::thread_rng;

pub trait CheckablySquare {
    fn is_square(&self) -> bool;
    fn root(&self) -> i32;
}

impl CheckablySquare for i32 {
    fn is_square(&self) -> bool {
        for i in 1..=(self / 2) {
            if i * i == *self {
                return true;
            }
        }
        return false;
    }

    fn root(&self) -> i32 {
        for i in 1..=(self / 2) {
            if i * i == *self {
                return i;
            }
        }
        return -1;
    }
}

pub struct SudokuBoard {
    pub values: Vec<Vec<Option<i32>>>,
    pub val_in_row: Vec<Vec<bool>>,
    pub val_in_col: Vec<Vec<bool>>,
    pub val_in_box: Vec<Vec<bool>>,
}

// Returns the number of the bold box associated with the given row and column coordinates
pub fn get_box_num(row: usize, col: usize, side_length: i32) -> usize {
    let bold_length = side_length.root();
    return ((bold_length * (row as i32 / bold_length)) + (col as i32 / bold_length)) as usize;
}

// Places a known value into the board and adjusts the possible values for everything else in its row, column, and bold square (the sqrt(n) x sqrt(n) square which this value is within)
pub fn update_board(board: &mut SudokuBoard, value: i32, row: usize, col: usize, side_length: i32) -> bool { // returns whether or not value added is possible
    let box_num: usize = get_box_num(row, col, side_length);
    if board.val_in_row[(value - 1) as usize][row] || board.val_in_col[(value - 1) as usize][col] || board.val_in_box[(value - 1) as usize][box_num] {
        return false;
    } else {
        board.val_in_row[(value - 1) as usize][row] = true;
        board.val_in_col[(value - 1) as usize][col] = true;
        board.val_in_box[(value - 1) as usize][box_num] = true;
        board.values[row][col] = Some(value);
        return true;
    }
}

// Removes value placed into the board as a guess
pub fn remove_val(board: &mut SudokuBoard, value: i32, row: usize, col: usize, side_length: i32) -> bool { // returns if it was successfully removed (right now it is always true)
    let box_num: usize = get_box_num(row, col, side_length);
    board.val_in_row[(value - 1) as usize][row] = false;
    board.val_in_col[(value - 1) as usize][col] = false;
    board.val_in_box[(value - 1) as usize][box_num] = false;
    board.values[row][col] = None;
    return true;
}

// Checks if every row, col, and bold box has every number 1 to side_length inclusive
pub fn is_board_solved(board: &SudokuBoard, side_length: i32) -> i32 { // returns integer code, 1 is solved, 0 means more work required, -1 means failure
    let mut row_check = vec![vec![false; side_length as usize]; side_length as usize];
    let mut col_check = vec![vec![false; side_length as usize]; side_length as usize];
    let mut box_check = vec![vec![false; side_length as usize]; side_length as usize];

    for row in 0..side_length {
        for col in 0..side_length {
            let entry = board.values[row as usize][col as usize];
            if entry == None {
                return 0; // puzzle is in progress
            } else {
                let val = entry.unwrap();
                row_check[row as usize][(val - 1) as usize] = true;
                col_check[col as usize][(val - 1) as usize] = true;
                let box_num: usize = get_box_num(row as usize, col as usize, side_length);
                box_check[box_num][(val - 1) as usize] = true;
            }
        }
    }
    for i in 0..side_length {
        for j in 0..side_length {
            if !(row_check[i as usize][j as usize] && col_check[i as usize][j as usize] && box_check[i as usize][j as usize]) {
                return -1; // puzzle is filled in, but incorrectly
            }
        }
    }
    return 1; // if it makes it to this point without returning, that means all criteria are satisfied
}

// Returns a vector of possible values for a given square on the sudoku board
pub fn possible_vals(board: &SudokuBoard, row: usize, col: usize, box_num: usize, side_length: i32) -> Vec<i32> {
    let mut possibilities: Vec<i32> = Vec::new();
    for v in 0..side_length {
        if !board.val_in_row[v as usize][row] && !board.val_in_col[v as usize][col] && !board.val_in_box[v as usize][box_num] {
            possibilities.push(v + 1)
        }
    }
    return possibilities;
}

// Attempts to solve the board by guessing possible values, checking if it produces a valid solution, recursively if necessary, then guessing again if not.
// Once all possible guesses are exhausted, admits defeat and returns false.
pub fn solve_board(board: &mut SudokuBoard, side_length: i32) -> bool {
    /*for row in 0..side_length {
        for col in 0..side_length {
            if board.values[row as usize][col as usize] != None {
                print!("| {} |", board.values[row as usize][col as usize].unwrap());
            } else {
                print!("|   |");
            }
        }
        println!();
    }*/
    //println!("solve board called");
    let initial_solve_result = is_board_solved(board, side_length);
    if initial_solve_result == 1 {
        return true;
    } else if initial_solve_result == -1 {
        return false;
    }

    let bold_length = side_length.root();

    // first check if any squares only have one possible value, and fill it in if they do
    for r in 0..side_length {
        for c in 0..side_length {
            if board.values[r as usize][c as usize] == None {
                let box_num: usize = get_box_num(r as usize, c as usize, side_length);
                let vs = possible_vals(board, r as usize, c as usize, box_num, side_length);
                let len = vs.len();
                if len == 0 {
                    return false; // if the number of possible values for an empty square is ever 0, the board cannot be solved
                } else if len == 1 {
                    if !update_board(board, vs[0], r as usize, c as usize, side_length) {
                        remove_val(board, vs[0], r as usize, c as usize, side_length);
                        return false; // if the only possible value is not valid, then the board cannot be solved
                    } else {
                        if !solve_board(board, side_length) {
                            remove_val(board, vs[0], r as usize, c as usize, side_length);
                            return false;
                        } else {
                            return true;
                        }
                    }
                }
            }
        }
    }

    // second check if nothing else in the same row, column, or box could have a possible value for the each square
    for r in 0..side_length {
        for c in 0..side_length {
            if board.values[r as usize][c as usize] == None {
                let box_num: usize = get_box_num(r as usize, c as usize, side_length);
                let vs = possible_vals(board, r as usize, c as usize, box_num, side_length);
                for v in vs {
                    let mut rest_of_row_could_have = false;
                    let mut rest_of_col_could_have = false;
                    let mut rest_of_box_could_have = false;
                    for i in 0..side_length {
                        if i != c && !rest_of_row_could_have {
                            let row_test = board.values[r as usize][i as usize];
                            if row_test == None {
                                if !board.val_in_col[(v - 1) as usize][i as usize] && !board.val_in_box[(v - 1) as usize][get_box_num(r as usize, i as usize, side_length)] {
                                    // we can assume that !board.val_in_row[(v - 1) as usize][r as usize] will return true because otherwise v would not have been a possible value for this square (which is in the same row)
                                    rest_of_row_could_have = true;
                                }
                            } else if row_test == Some(v) {
                                rest_of_row_could_have = true;
                            }
                        }
                        if i != r && !rest_of_col_could_have {
                            let col_test = board.values[i as usize][c as usize];
                            if col_test == None {
                                if !board.val_in_row[(v - 1) as usize][i as usize] && !board.val_in_box[(v - 1) as usize][get_box_num(i as usize, c as usize, side_length)] {
                                    // we can assume that !board.val_in_col[(v - 1) as usize][c as usize] will return true because otherwise v would not have been a possible value for this square (which is in the same col)
                                    rest_of_col_could_have = true;
                                }
                            } else if col_test == Some(v) {
                                rest_of_col_could_have = true;
                            }
                        }
                    }
                    if !rest_of_row_could_have || !rest_of_col_could_have { // nothing else in either the row or column could be a possible value for this one, so this square must be that value
                        if !update_board(board, v, r as usize, c as usize, side_length) {
                            remove_val(board, v, r as usize, c as usize, side_length);
                            return false; // this square must be a value that results in an invalid board
                        } else {
                            if !solve_board(board, side_length) {
                                remove_val(board, v, r as usize, c as usize, side_length);
                                return false;
                            } else {
                                return true;
                            }
                        }
                    } else {
                        let box_left_col = bold_length * (c / bold_length);
                        let box_right_col = box_left_col + bold_length - 1;
                        let box_top_row = bold_length * (r / bold_length);
                        let box_bot_row = box_top_row + bold_length - 1;
                        for row in box_top_row..=box_bot_row {
                            for col in box_left_col..=box_right_col {
                                if !rest_of_box_could_have && (row != r || col != c) { // equivalent to !(r == row && c == col), but in theory because we just check the same row and column, this could be an AND
                                    let box_test = board.values[row as usize][col as usize];
                                    if box_test == None {
                                        if !board.val_in_row[(v - 1) as usize][row as usize] && !board.val_in_col[(v - 1) as usize][col as usize] {
                                            // we can assume that !board.val_in_box[(v - 1) as usize][c as usize] will return true because otherwise v would not have been a possible value for this square (which is in the same box)
                                            rest_of_box_could_have = true;
                                        }
                                    } else if box_test == Some(v) {
                                        rest_of_box_could_have = true;
                                    }
                                }
                            }
                        }
                        if !rest_of_box_could_have {
                            if !update_board(board, v, r as usize, c as usize, side_length) {
                                remove_val(board, v, r as usize, c as usize, side_length);
                                return false; // this square must be a value that results in an invalid board
                            } else {
                                if !solve_board(board, side_length) {
                                    remove_val(board, v, r as usize, c as usize, side_length);
                                    return false;
                                } else {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // third guess possible values for each square and see if they could work (guesses on square with the fewest possible values first)
    let mut rng = thread_rng();
    let mut min_len = side_length + 1;
    let mut min_vs: Vec<i32> = vec![0];
    let mut min_info: (i32, i32) = (0,0); // row, col
    for r in 0..side_length {
        for c in 0..side_length {
            if board.values[r as usize][c as usize] == None {
                let box_num: usize = get_box_num(r as usize, c as usize, side_length);
                let mut vs = possible_vals(board, r as usize, c as usize, box_num, side_length);
                let len = vs.len();
                if len == 2 { // the lowest possible number of values to guess from is 2, so if we find this we immediately do the guess
                    vs.shuffle(&mut rng);
                    for guess_v in vs {
                        // place the value in the board, see if it could work
                        if !update_board(board, guess_v, r as usize, c as usize, side_length) {
                            return false; // guess value was not valid, this should never happen
                        } else {
                            if solve_board(board, side_length) {
                               return true;
                            } else {
                               remove_val(board, guess_v, r as usize, c as usize, side_length);
                            }
                        }
                   }
                   return false; // if it deemed that none of the possible values for any given square could lead to a solvable board, then the board is unsolvable
                } else if (len as i32) < min_len {
                    min_len = len as i32;
                    min_vs = vs.clone();
                    min_info = (r, c);
                }
            }
        }
    }
    let (r, c) = min_info;
    min_vs.shuffle(&mut rng);
    for guess_v in min_vs {
        // place the value in the board, see if it could work
        if !update_board(board, guess_v, r as usize, c as usize, side_length) {
            return false; // guess value was not valid, this should never happen
        } else {
            if solve_board(board, side_length) {
                return true;
            } else {
                remove_val(board, guess_v, r as usize, c as usize, side_length);
            }
        }
    }
    return false; // if it deemed that none of the possible values for any given square could lead to a solvable board, then the board is unsolvable
}