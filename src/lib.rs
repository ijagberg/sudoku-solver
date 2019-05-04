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

    pub fn is_solved(&self) -> bool {
        for v in 1..=self.size {
            // Check rows
            for row in 0..self.height() {
                if self.count_in_row(row, v as u32) != 1 {
                    return false;
                }
            }

            // Check cols
            for col in 0..self.width() {
                if self.count_in_col(col, v as u32) != 1 {
                    return false;
                }
            }

            // Check secs
            for row in (0..self.height()).step_by(self.sec_height) {
                for col in (0..self.width()).step_by(self.sec_width) {
                    if self.count_in_sec(row, col, v as u32) != 1 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn count_in_row(&self, row: usize, value: u32) -> usize {
        self.grid[row]
            .iter()
            .filter(|&&value_in_row| value_in_row == Some(value))
            .count()
    }

    fn count_in_col(&self, col: usize, value: u32) -> usize {
        self.grid
            .iter()
            .map(|row| row[col])
            .filter(|&value_in_col| value_in_col == Some(value))
            .count()
    }

    fn count_in_sec(&self, row: usize, col: usize, value: u32) -> usize {
        let first_row_in_sec = (row / self.sec_height) * self.sec_height;
        let first_col_in_sec = (col / self.sec_width) * self.sec_width;
        let mut sec_count = 0;
        for sec_row in first_row_in_sec..first_row_in_sec + self.sec_height {
            for sec_col in first_col_in_sec..first_col_in_sec + self.sec_width {
                if self.grid[sec_row][sec_col] == Some(value) {
                    sec_count += 1;
                }
            }
        }
        sec_count
    }

    fn can_place_value(&self, value: u32, row: usize, col: usize) -> bool {
        self.grid[row][col] == None
            && self.count_in_row(row, value) == 0
            && self.count_in_col(col, value) == 0
            && self.count_in_sec(row, col, value) == 0
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
        use std::iter;
        let top_border = format!(
            "┌{}┐",
            iter::repeat("─")
                .take(self.width() * 2 - 1)
                .collect::<String>(),
        );
        writeln!(f, "{}", top_border)?;

        let mut row = 0;
        while row < self.height() {
            if row > 0 && row % self.sec_height == 0 {
                writeln!(
                    f,
                    "{}",
                    iter::repeat("─")
                        .take(self.width() * 2 - 1)
                        .collect::<String>()
                )?;
            }
            let row_output = self.grid[row]
                .iter()
                .enumerate()
                .map(|(col_index, col_value)| {
                    let col_output = match col_value {
                        Some(c) => c.to_string(),
                        None => " ".to_string(),
                    };
                    if col_index > 0 && col_index % self.sec_width == 0 {
                        format!("│ {}", col_output)
                    } else {
                        format!("{}", col_output)
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(f, "│ {} │", row_output)?;
            row += 1;
        }

        let bottom_border = format!(
            "└{}└",
            iter::repeat("─")
                .take(self.width() * 2 - 1)
                .collect::<String>(),
        );
        writeln!(f, "{}", bottom_border)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_solvable_9x9_test_instance() -> Sudoku {
        // 003020600
        // 900305001
        // 001806400
        // 008102900
        // 700000008
        // 006708200
        // 002609500
        // 800203009
        // 005010300
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

    fn get_unsolvable_9x9_test_instance() -> Sudoku {
        // 003020600
        // 900305001
        // 001806400
        // 008102900
        // 700000008
        // 006708200
        // 002609500
        // 800203309
        // 005010000
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

    fn get_solvable_6x6_test_instance() -> Sudoku {
        Sudoku {
            size: 6,
            sec_width: 3,
            sec_height: 2,
            grid: vec![
                vec![Some(4), Some(2), Some(1), Some(3), None, None],
                vec![None, None, Some(5), Some(4), None, Some(1)],
                vec![None, None, Some(2), Some(6), None, None],
                vec![None, Some(6), Some(4), None, None, None],
                vec![Some(2), None, Some(6), Some(1), None, Some(3)],
                vec![Some(1), Some(5), Some(3), Some(2), Some(4), None],
            ],
        }
    }

    #[test]
    fn test_solvable_9x9() {
        let mut solvable_test_instance = get_solvable_9x9_test_instance();
        assert!(!solvable_test_instance.is_solved());
        solvable_test_instance.solve();
        assert!(solvable_test_instance.is_solved());
    }

    #[test]
    fn test_unsolvable_9x9() {
        let unsolvable_test_instance = get_unsolvable_9x9_test_instance();
        assert!(!unsolvable_test_instance.is_solved());
        let solved_unsolvable_test_instance = unsolvable_test_instance.solved();
        assert!(!solved_unsolvable_test_instance.is_solved());
        assert!(unsolvable_test_instance == solved_unsolvable_test_instance);
    }

    #[test]
    fn test_solvable_6x6() {
        let mut solvable_test_instance = get_solvable_6x6_test_instance();
        assert!(!solvable_test_instance.is_solved());
        solvable_test_instance.solve();
        assert!(solvable_test_instance.is_solved());
        println!("{:?}", solvable_test_instance);
    }
}
