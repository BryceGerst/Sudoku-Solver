use std::io::{self, Write, BufRead};

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

// Places a known value into the board and adjusts the possible values for everything else in its row, column, and bold square (the sqrt(n) x sqrt(n) square which this value is within)
pub fn update_board(board: &mut Vec<Vec<Vec<i32>>>, value: i32, row: usize, col: usize, side_length: i32) -> bool { // returns whether or not value added is possible
    board[row][col] = vec![value];
    /*println!("called");
    for r in 0..side_length {
        for c in 0..side_length {
            //print!("| {:?} |", board[r as usize][c as usize]);
            if board[r as usize][c as usize].len() == 1 {
                print!("| {} |", board[r as usize][c as usize][0]);
            } else {
                print!("|   |");
            }
        }
        println!();
    }*/
    // removes value from possible options for all other board pieces in the same row and column
    for i in 0..side_length {
        if i != col as i32 {
            let col_index = board[row][i as usize].iter().position(|&e| e == value);
            if col_index != None {
                board[row][i as usize].remove(col_index.unwrap());
                // recursively calls itself if while changing possibilities it discovers that a square must be some value
                if board[row][i as usize].len() == 1 {
                    if !update_board(board, board[row][i as usize][0], row, i as usize, side_length) {
                        return false;
                    }
                } else if board[row][i as usize].len() == 0 {
                    return false; // board is impossible
                }
            }
        }
        if i != row as i32 {
            let row_index = board[i as usize][col].iter().position(|&e| e == value);
            if row_index != None {
                board[i as usize][col].remove(row_index.unwrap());
                // recursively calls itself if while changing possibilities it discovers that a square must be some value
                if board[i as usize][col].len() == 1 {
                    if !update_board(board, board[i as usize][col][0], i as usize, col, side_length) {
                        return false;
                    }
                } else if board[i as usize][col].len() == 0 {
                    return false; // board is impossible
                }
            }
        }
    }
    // removes value from possible options for all other board pieces in the same bold square
    let bold_length = side_length.root();
    let box_left_col = bold_length * ((col as i32) / bold_length);
    let box_right_col = box_left_col + bold_length - 1;
    let box_top_row = bold_length * ((row as i32) / bold_length);
    let box_bot_row = box_top_row + bold_length - 1;
    for r in box_top_row..=box_bot_row {
        for c in box_left_col..=box_right_col {
            if r != row as i32 && c != col as i32 { // technically would use OR here if this was standalone, but since we already removed from the same row and col we can use AND as a slight optimization
                let index = board[r as usize][c as usize].iter().position(|&e| e == value);
                if index != None {
                    board[r as usize][c as usize].remove(index.unwrap());
                    // recursively calls itself if while changing possibilities it discovers that a square must be some value
                    if board[r as usize][c as usize].len() == 1 {
                        if !update_board(board, board[r as usize][c as usize][0], r as usize, c as usize, side_length) {
                            return false;
                        }
                    } else if board[r as usize][c as usize].len() == 0 {
                        return false; // board is impossible
                    }
                }
            }
        }
    }
    return true;
}

// Checks if every row, col, and bold box has every number 1 to side_length inclusive
pub fn is_board_solved(board: &Vec<Vec<Vec<i32>>>, side_length: i32) -> i32 { // returns integer code, 1 is solved, 0 means more work required, -1 means failure
    let mut row_check = vec![vec![false; side_length as usize]; side_length as usize];
    let mut col_check = vec![vec![false; side_length as usize]; side_length as usize];
    let mut box_check = vec![vec![false; side_length as usize]; side_length as usize];
    let bold_length = side_length.root();

    for row in 0..side_length {
        for col in 0..side_length {
            let length = board[row as usize][col as usize].len();
            if length == 0 {
                return -1; // puzzle has been deemed impossible
            } else if length == 1 {
                row_check[row as usize][(board[row as usize][col as usize][0] - 1) as usize] = true;
                col_check[col as usize][(board[row as usize][col as usize][0] - 1) as usize] = true;
                let box_num: i32 = (bold_length * (row / bold_length)) + (col / bold_length);
                box_check[box_num as usize][(board[row as usize][col as usize][0] - 1) as usize] = true;
            } else {
                return 0; // puzzle is not filled in all the way, still some unknowns
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

// Attempts to solve the board by guessing possible values, checking if it produces a valid solution, recursively if necessary, then guessing again if not.
// Once all possible guesses are exhausted, admits defeat and returns false.
pub fn solve_board(board: &mut Vec<Vec<Vec<i32>>>, side_length: i32) -> bool {
    //println!("solve board called");
    let initial_solve_result = is_board_solved(board, side_length);
    if initial_solve_result == 1 {
        return true;
    } else if initial_solve_result == -1 {
        return false;
    }

    let bold_length = side_length.root();

    for row in 0..side_length {
        for col in 0..side_length {
            let length = board[row as usize][col as usize].len();
            if length > 1 {
                let mut found_value = None;
                for possible_val in &board[row as usize][col as usize] {
                    if found_value == None {
                        let mut rest_of_row_could_have = false;
                        let mut rest_of_col_could_have = false;
                        let mut rest_of_box_could_have = false;
                        for i in 0..side_length {
                            if i != col && board[row as usize][i as usize].contains(possible_val) {
                                rest_of_row_could_have = true;
                            }
                            if i != row && board[i as usize][col as usize].contains(possible_val) {
                                rest_of_col_could_have = true;
                            }
                        }
                        if !rest_of_row_could_have || !rest_of_col_could_have {
                            /*if !rest_of_row_could_have {
                                println!("no way it can be elsewhere in row {}", (row+1));
                            }
                            if !rest_of_col_could_have {
                                println!("no way it can be elsewhere in col {}", (col+1));
                            }*/
                            found_value = Some(*possible_val);
                        } else {
                            let box_left_col = bold_length * (col / bold_length);
                            let box_right_col = box_left_col + bold_length - 1;
                            let box_top_row = bold_length * (row / bold_length);
                            let box_bot_row = box_top_row + bold_length - 1;
                            for r in box_top_row..=box_bot_row {
                                for c in box_left_col..=box_right_col {
                                    if r != row || c != col { // equivalent to !(r == row && c == col)
                                        if board[r as usize][c as usize].contains(possible_val) {
                                            rest_of_box_could_have = true;
                                        }
                                    }
                                }
                            }
                            if !rest_of_box_could_have {
                                //println!("no way it can be elsewhere in box spanning rows {} through {} cols {} through {}", (box_top_row+1), (box_bot_row+1), (box_left_col+1), (box_right_col+1));
                                found_value = Some(*possible_val);
                            }
                        }
                    }
                }
                if found_value != None {
                    //println!("Determined that row {}, col {} must have value {}", (row+1), (col+1), found_value.unwrap());
                    if !update_board(board, found_value.unwrap(), row as usize, col as usize, side_length) {
                        return false;
                    } else{
                        return solve_board(board, side_length); // we want to repeat the process from the start as soon as any value is determined
                    }
                }

            }
        }
    }
    // after this point values for squares are guessed based on current possibilities
    //println!("hit guessing stage");

    for row in 0..side_length {
        for col in 0..side_length {
            let length = board[row as usize][col as usize].len();
            if length > 1 {
                let mut bad_val = None;
                for guess_val in &board[row as usize][col as usize] {
                    if bad_val == None {
                        let mut test_board = board.clone();
                        /*println!("----- guessing {} in row {} col {} now that entry is {:?} --------", *guess_val, row, col, test_board[row as usize][col as usize]);
                        for r in 0..side_length {
                            for c in 0..side_length {
                                if test_board[r as usize][c as usize].len() == 1 {
                                    print!("| {} |", test_board[r as usize][c as usize][0]);
                                } else {
                                    print!("|   |");
                                }
                            }
                            println!();
                        }*/
                        if !update_board(&mut test_board, *guess_val, row as usize, col as usize, side_length) {
                            //println!("we know {} was not legitimate", *guess_val);
                            bad_val = Some(*guess_val);
                        } else {
                            if solve_board(&mut test_board, side_length) {
                                *board = test_board.clone();
                                return true;
                            } else { // now we should remove guess_val as a possibility (since we have determined there is no way for it to work), and start from the top
                                bad_val = Some(*guess_val);
                            }
                        }
                    }
                }
                if bad_val != None {
                    //println!("eliminated {}", bad_val.unwrap());
                    let remove_index = board[row as usize][col as usize].iter().position(|&e| e == bad_val.unwrap()).unwrap();
                    board[row as usize][col as usize].remove(remove_index);
                    let remaining_length = board[row as usize][col as usize].len();
                    if remaining_length == 0 {
                        return false;
                    } else if remaining_length == 1 {
                        if !update_board(board, board[row as usize][col as usize][0], row as usize, col as usize, side_length) {
                            return false;
                        }
                    }
                    return solve_board(board, side_length);
                }
            }
        }
    }
    return false; // if we have hit this point that means no chain of guesses resulted in a working solution
}

/*
// main function which runs on program execution
fn main() {
    let stdin = io::stdin();
    let mut side_length: i32;

    // gather board dimensions from the user
    loop {
        print!("Input board side length (should be a positive integer which is a perfect square): ");
        io::stdout().flush().unwrap();

        // input code from: https://www.reddit.com/r/rust/comments/41hgwq/help_needed_user_input_in_rust/
        let mut lines = stdin.lock().lines().fuse();

        let input = match lines.next() {
            Some(Ok(a)) => a,
            _ => panic!("Failed to read input.")
        };
        // by user: jakko100

        let result = input.parse();
        side_length = match result {
            Result::Ok(num) => num,
            _ => -1,
        };
        if side_length > 0 && side_length.is_square() { break; }
    }

    // generates the board with possible values for each square (starts with all being between 1 and the square root of the side length)
    let mut board = vec![vec![(1..=side_length).collect::<Vec<i32>>(); side_length as usize]; side_length as usize];

    // gather known values from the user and their positions
    loop {
        let value: i32;
        let row: i32;
        let col: i32;
        let mut input;
        {
            print!("Enter known value (should be between 1 and {}) (type stop when finished): ", side_length);
            io::stdout().flush().unwrap();

            let mut lines = stdin.lock().lines().fuse();

            input = match lines.next() {
                Some(Ok(a)) => a,
                _ => panic!("Failed to read input.")
            };

            let result = input.parse();
            value = match result {
                Result::Ok(num) => if num > side_length {-1} else {num},
                _ => -1,
            };
        }
        if input == "stop".to_string() { break; }
        {
            print!("What row is it in? (should be between 1 and {}): ", side_length);
            io::stdout().flush().unwrap();

            let mut lines = stdin.lock().lines().fuse();

            input = match lines.next() {
                Some(Ok(a)) => a,
                _ => panic!("Failed to read input.")
            };

            if input == "STOP".to_string() { break; }

            let result = input.parse();
            row = match result {
                Result::Ok(num) => if num > side_length {-1} else {num},
                _ => -1,
            };
        }
        {
            print!("What column is it in? (should be between 1 and {}): ", side_length);
            io::stdout().flush().unwrap();

            let mut lines = stdin.lock().lines().fuse();

            input = match lines.next() {
                Some(Ok(a)) => a,
                _ => panic!("Failed to read input.")
            };

            if input == "STOP".to_string() { break; }

            let result = input.parse();
            col = match result {
                Result::Ok(num) => if num > side_length {-1} else {num},
                _ => -1,
            };
        }

        if row >= 1 && row <= side_length && col >= 1 && col <= side_length && value > 0 && value <= side_length {
            update_board(&mut board, value, (row - 1) as usize, (col - 1) as usize, side_length); // begins to solve the board as you input data
        } else {
            println!("Invalid input");
        }
    }

    // solves the puzzle / determines if it is solvable
    println!("{:?}", board);
    if solve_board(&mut board, side_length) {
        println!("Solved the board!");
        for row in 0..side_length {
            for col in 0..side_length {
                print!("| {} |", board[row as usize][col as usize][0])
            }
        println!();
        }
    } else {
        println!("{:?}", board);
        println!("Unable to solve the board.");
    }
    /*
    let initial_solve_result = is_board_solved(&board, side_length);
    if initial_solve_result == 0 {
        if !solve_board(&mut board, side_length) {
            println!("Unable to solve the board after making several attempts");
            println!("{:?}", board);
        } else {
            println!("Board is solved! (required signficant work)");
            for row in 0..side_length {
                for col in 0..side_length {
                    print!("| {} |", board[row as usize][col as usize][0])
                }
            println!();
            }
        }
    } else if initial_solve_result == 1 {
        println!("Board is solved!");
        for row in 0..side_length {
            for col in 0..side_length {
                print!("| {} |", board[row as usize][col as usize][0])
            }
        println!();
        }
    } else {
        println!("Unable to solve the board")
    }
    */   
}
*/