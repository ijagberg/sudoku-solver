use sudoku::Sudoku;

pub trait Solve {
    fn solve(puzzle: Sudoku) -> Result<Sudoku, SolveError>;
}

#[derive(Debug)]
pub enum SolveError {
    Unsolvable,
}

pub struct Backtrack;

impl Backtrack {
    pub fn solve_backtrack(puzzle: &mut Sudoku) -> bool {
        for row in 0..puzzle.height() {
            for col in 0..puzzle.width() {
                if puzzle.get(col, row).is_none() {
                    for v in 1..=puzzle.size() {
                        if puzzle.can_place_value(col, row, v as u32) {
                            puzzle.set(col, row, Some(v as u32));
                            if Backtrack::solve_backtrack(puzzle) {
                                return true;
                            } else {
                                puzzle.set(col, row, None);
                            }
                        }
                    }
                    return false;
                }
            }
        }
        true
    }
}

impl Solve for Backtrack {
    fn solve(mut puzzle: Sudoku) -> Result<Sudoku, SolveError> {
        match Backtrack::solve_backtrack(&mut puzzle) {
            true => Ok(puzzle),
            false => Err(SolveError::Unsolvable),
        }
    }
}
