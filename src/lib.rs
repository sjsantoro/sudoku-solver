//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use rand::{thread_rng, Rng};

#[derive(Debug)]
struct EmptyCell {
  row: usize,
  col: usize,
}

fn get_subcell_root(row: usize, col: usize) -> (usize, usize) {
  (row / 3 * 3, col / 3 * 3)
}

fn valid_position(num: char, sub_cell_x: usize, sub_cell_y: usize, row: usize, col: usize, board: &Vec<Vec<char>>) -> bool {
    for y in sub_cell_y..sub_cell_y+3 {
        for x in sub_cell_x..sub_cell_x+3 {

            // Do not check itself.
            if x == row && y == col {
              continue;
            }

            if board[x][y] == num {
                return false;
            }
        }
    }
    
    // Check if number is in the row
    for x in 0..9 {

          // Do not check itself.
          if x == row {
            continue;
          }

          if num == board[x][col] {
              return false;
          }
    }

    // Check if number is in the col
    for y in 0..9 {

          // Do not check itself.
          if y == col {
            continue;
          }

          if num == board[row][y] {
              return false;
          }
    }
    
    true
}

fn calculate_value(idx: usize, cells: &Vec<EmptyCell>, board: &mut Vec<Vec<char>>) -> bool {
  let cell = &cells[idx];
  let next = idx + 1;
  
  // Calculate sub-cell origins
  let sub_cell_x = cell.row / 3 * 3;
  let sub_cell_y = cell.col / 3 * 3;
  
  let mut cur: u8 = '1' as u8;
  
  loop {
      if valid_position(cur as char, sub_cell_x, sub_cell_y, cell.row, cell.col, &*board) {
          board[cell.row][cell.col] = cur as char;

          if next == cells.len() {
              return true;
          }
          
          match calculate_value(next, &cells, board) {
              true => break,
              false => {
                  if cur == '9' as u8 {
                      board[cell.row][cell.col] = '.';
                      return false;
                  };
                  
                  cur += 1;      
              },
          }
      } else {
          if cur == '9' as u8 {
              board[cell.row][cell.col] = '.';
              return false;
          }
          
          cur += 1;
      }
  }
  true
}

/// Generates a puzzle
///
/// # Arguments
///
/// * `min` - Minimum numbers in the generated puzzle.
/// * `max` - Maximum amount of numbers in the generated puzzle.
///
/// * Returns a 2D char vector containing the cell numbers as chars. Empty cells are denoted with a `.`.
pub fn generate_puzzle(min: u32, max: u32) -> Vec<Vec<char>> {
    
    // Create an empty board
    let mut board = vec![vec!['.'; 9]; 9];

    let mut rng = thread_rng();

    // Random board numbers to generate
    let numbers_to_generate: u32 = rng.gen_range(min, max);

    // Loop through numbers and randomly choose x and y values for each.
    'outer: for _ in 0..numbers_to_generate {
        loop {
            let row = rng.gen_range(0, 9);
            let col = rng.gen_range(0, 9);

            // Ensure that the value in the board is empty.
            if board[row][col] != '.' {
                // Go back to top and pick another value.
                continue;
            }

            loop {
                board[row][col] = rng.gen_range(49, 58) as u8 as char;

                // Ensure puzzle is valid.
                match validate_puzzle(&board) {
                    Err(_) => continue,
                    _ => continue 'outer,
                }
            }
        }
    }

    board
}

/// Solves a sudoku puzzle.
///
/// # Arguments
///
/// * `board` - A 2D char vector containing the cell numbers as chars. Empty cells are denoted with a `.`.
///
pub fn solve_puzzle(board: &mut Vec<Vec<char>>) {
  let mut available_cells: Vec<EmptyCell> = Vec::with_capacity(9*9);
  
  for row in 0..9 {
      for col in 0..9 {
          let c = board[row][col];
          
          if c == '.' {
              available_cells.push(EmptyCell {
                  row,
                  col,
              })
          }
      }
  }
  
  calculate_value(0, &available_cells, board);
}

/// Validates that all the numbers on a board are valid.
/// Returns an error with the invalid column and row location.
///
/// # Arguments
///
/// * `board` - A 2D char vector containing the cell numbers as chars. Empty cells are denoted with a `.`.
///
pub fn validate_puzzle(board: &Vec<Vec<char>>) -> Result<(), (usize, usize)> {
  for row in 0..board.len() {
      for col in 0..board[row].len() {
          let c = board[row][col];
          let (root_x, root_y) = get_subcell_root(row, col);
          
          if c != '.' {
            if !valid_position(c, root_x, root_y, row, col, &*board) {
              return Err((row, col));
            }
          }
      }
  }

  Ok(())
}

fn print_puzzle(board: &Vec<Vec<char>>) {
    for row in board {
        print!("\n------------------\n|");

        for col in row {
            print!("{}|", col);
        }

        print!("\n------------------\n");
    }

}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_solver() {
        let mut input = vec![
          vec!['5','3','.','.','7','.','.','.','.'],
          vec!['6','.','.','1','9','5','.','.','.'],
          vec!['.','9','8','.','.','.','.','6','.'],
          vec!['8','.','.','.','6','.','.','.','3'],
          vec!['4','.','.','8','.','3','.','.','1'],
          vec!['7','.','.','.','2','.','.','.','6'],
          vec!['.','6','.','.','.','.','2','8','.'],
          vec!['.','.','.','4','1','9','.','.','5'],
          vec!['.','.','.','.','8','.','.','7','9'],
        ];

        let result = vec![
          vec!['5','3','4','6','7','8','9','1','2'],
          vec!['6','7','2','1','9','5','3','4','8'],
          vec!['1','9','8','3','4','2','5','6','7'],
          vec!['8','5','9','7','6','1','4','2','3'],
          vec!['4','2','6','8','5','3','7','9','1'],
          vec!['7','1','3','9','2','4','8','5','6'],
          vec!['9','6','1','5','3','7','2','8','4'],
          vec!['2','8','7','4','1','9','6','3','5'],
          vec!['3','4','5','2','8','6','1','7','9'],
        ];

        solve_puzzle(&mut input);

        assert_eq!(input, result);
    }

    #[test]
    fn test_generator() {
        let puzzle = generate_puzzle(20, 30);

        assert_eq!(validate_puzzle(&puzzle), Ok(()));
    }
}
