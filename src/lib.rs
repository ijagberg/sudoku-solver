#[derive(Clone, PartialEq)]
pub struct Sudoku {
    size: usize,
    sec_width: usize,
    sec_height: usize,
    grid: Vec<Vec<Option<u32>>>,
}

impl Sudoku {
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn value_in_row(&self, value: u32, row: usize) -> bool {
        self.grid[row]
            .iter()
            .any(|&value_in_row| value_in_row == Some(value))
    }

    fn value_in_col(&self, value: u32, col: usize) -> bool {
        self.grid
            .iter()
            .map(|row| row[col])
            .any(|value_in_col| value_in_col == Some(value))
    }

    fn value_in_sec(&self, value: u32, row: usize, col: usize) -> bool {
        let first_row_in_sec = (row / self.sec_height) * self.sec_height;
        let first_col_in_sec = (col / self.sec_width) * self.sec_width;
        for sec_row in first_row_in_sec..first_row_in_sec + self.sec_height {
            for sec_col in first_col_in_sec..first_col_in_sec + self.sec_width {
                if self.grid[sec_row][sec_col] == Some(value) {
                    return true;
                }
            }
        }
        false
    }

    fn can_place_value(&self, value: u32, row: usize, col: usize) -> bool {
        !self.value_in_row(value, row)
            && !self.value_in_col(value, col)
            && !self.value_in_sec(value, row, col)
    }

    pub fn solved(&self) -> Sudoku {
        let mut self_temp = self.clone();
        self_temp.solve();
        self_temp
    }

    pub fn solve(&mut self) {
        self.solve_backtrack();
    }

    fn solve_backtrack(&mut self) -> bool {
        for row in 0..self.height() {
            for col in 0..self.width() {
                if self.grid[row][col] == None {
                    for v in 1..=self.size {
                        if self.can_place_value(v as u32, row, col) {
                            self.grid[row][col] = Some(v as u32);
                            if self.solve_backtrack() {
                                return true;
                            } else {
                                self.grid[row][col] = None;
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

impl std::fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row
                    .iter()
                    .map(|c| match c {
                        Some(v) => v.to_string(),
                        None => "_".to_string(),
                    })
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 003020600
    // 900305001
    // 001806400
    // 008102900
    // 700000008
    // 006708200
    // 002609500
    // 800203009
    // 005010300

    fn get_solvable_test_instance() -> Sudoku {
        Sudoku {
            size: 9,
            sec_width: 3,
            sec_height: 3,
            grid: vec![
                vec![
                    None,
                    None,
                    Some(3),
                    None,
                    Some(2),
                    None,
                    Some(6),
                    None,
                    None,
                ],
                vec![
                    Some(9),
                    None,
                    None,
                    Some(3),
                    None,
                    Some(5),
                    None,
                    None,
                    Some(1),
                ],
                vec![
                    None,
                    None,
                    Some(1),
                    Some(8),
                    None,
                    Some(6),
                    Some(4),
                    None,
                    None,
                ],
                vec![
                    None,
                    None,
                    Some(8),
                    Some(1),
                    None,
                    Some(2),
                    Some(9),
                    None,
                    None,
                ],
                vec![Some(7), None, None, None, None, None, None, None, Some(8)],
                vec![
                    None,
                    None,
                    Some(6),
                    Some(7),
                    None,
                    Some(8),
                    Some(2),
                    None,
                    None,
                ],
                vec![
                    None,
                    None,
                    Some(2),
                    Some(6),
                    None,
                    Some(9),
                    Some(5),
                    None,
                    None,
                ],
                vec![
                    Some(8),
                    None,
                    None,
                    Some(2),
                    None,
                    Some(3),
                    None,
                    None,
                    Some(9),
                ],
                vec![
                    None,
                    None,
                    Some(5),
                    None,
                    Some(1),
                    None,
                    Some(3),
                    None,
                    None,
                ],
            ],
        }
    }

    fn get_unsolvable_test_instance() -> Sudoku {
        Sudoku {
            size: 9,
            sec_width: 3,
            sec_height: 3,
            grid: vec![
                vec![
                    None,
                    None,
                    Some(3),
                    None,
                    Some(2),
                    None,
                    Some(6),
                    None,
                    None,
                ],
                vec![
                    Some(9),
                    None,
                    None,
                    Some(3),
                    None,
                    Some(5),
                    None,
                    None,
                    Some(1),
                ],
                vec![
                    None,
                    None,
                    Some(1),
                    Some(8),
                    None,
                    Some(6),
                    Some(4),
                    None,
                    None,
                ],
                vec![
                    None,
                    None,
                    Some(8),
                    Some(1),
                    None,
                    Some(2),
                    Some(9),
                    None,
                    None,
                ],
                vec![Some(7), None, None, None, None, None, None, None, Some(8)],
                vec![
                    None,
                    None,
                    Some(6),
                    Some(7),
                    None,
                    Some(8),
                    Some(2),
                    None,
                    None,
                ],
                vec![
                    None,
                    None,
                    Some(2),
                    Some(6),
                    None,
                    Some(9),
                    Some(5),
                    None,
                    None,
                ],
                vec![
                    Some(8),
                    None,
                    None,
                    Some(2),
                    None,
                    Some(3),
                    Some(3),
                    None,
                    Some(9),
                ],
                vec![None, None, Some(5), None, Some(1), None, None, None, None],
            ],
        }
    }

    #[test]
    fn solve() {
        let mut test_instance = get_solvable_test_instance();
        println!("Before solve:");
        println!("{:?}", test_instance);
        test_instance.solve();
        println!("After solve:");
        println!("{:?}", test_instance);
    }

    #[test]
    fn solved_fail() {
        let unsolvable_test_instance = get_unsolvable_test_instance();
        let solved_unsolvable_test_instance = unsolvable_test_instance.solved();
        assert!(unsolvable_test_instance == solved_unsolvable_test_instance);
    }
}
