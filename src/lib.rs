#[derive(Clone, PartialEq)]
pub struct Sudoku {
    size: usize,
    sec_width: usize,
    sec_height: usize,
    grid: Vec<Vec<Option<u32>>>,
}

pub struct ExactCover {
    grid: Vec<Vec<bool>>,
}

impl ExactCover {
    pub fn from(sudoku: Sudoku) -> ExactCover {
        let mut exact_cover = ExactCover {
            grid: vec![
                vec![false; sudoku.size * sudoku.size * 4];
                sudoku.size * sudoku.size * sudoku.size
            ],
        };
        exact_cover
    }
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
        let value_length = self.size.to_string().len();

        let row_outputs: Vec<String> = {
            (0..self.height())
                .map(|row| {
                    self.grid[row]
                        .iter()
                        .enumerate()
                        .flat_map(|(col_index, col_value)| {
                            let col_value_output = match col_value {
                                Some(c) => iter::repeat(" ".to_string())
                                    .take(value_length - c.to_string().len())
                                    .chain(iter::once(c.to_string()))
                                    .collect::<String>(),
                                None => iter::repeat("─").take(value_length).collect::<String>(),
                            };
                            if col_index > 0 && col_index % self.sec_width == 0 {
                                // Border between two secs
                                vec!["│".to_string(), col_value_output]
                            } else {
                                vec![col_value_output]
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .map(|row_string| format!("│ {} │", row_string))
                .collect::<Vec<String>>()
        };

        let mut row = 0;
        let row_length = row_outputs[0].chars().count();
        while row < self.height() {
            if row % self.sec_height == 0 {
                if row == 0 {
                    // Top border
                    writeln!(
                        f,
                        "{}",
                        row_outputs[0]
                            .chars()
                            .enumerate()
                            .map(|(idx, c)| match c {
                                '│' if idx == 0 => '┌',
                                '│' if idx == row_length - 1 => '┐',
                                '│' => '┬',
                                _ => '─',
                            })
                            .collect::<String>()
                    )?;
                } else {
                    // Middle border(s)
                    writeln!(
                        f,
                        "{}",
                        row_outputs[0]
                            .chars()
                            .enumerate()
                            .map(|(idx, c)| match c {
                                '│' if idx == 0 => '├',
                                '│' if idx == row_length - 1 => '┤',
                                '│' => '┼',
                                _ => '─',
                            })
                            .collect::<String>()
                    )?;
                }
            }

            writeln!(f, "{}", row_outputs[row])?;
            row += 1;

            if row % self.sec_height == 0 && row == self.height() {
                // Bottom border
                writeln!(
                    f,
                    "{}",
                    row_outputs[0]
                        .chars()
                        .enumerate()
                        .map(|(idx, c)| match c {
                            '│' if idx == 0 => '└',
                            '│' if idx == row_length - 1 => '┘',
                            '│' => '┴',
                            _ => '─',
                        })
                        .collect::<String>()
                )?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_solvable_12x12_test_instance() -> Sudoku {
        Sudoku {
            size: 12,
            sec_width: 4,
            sec_height: 3,
            grid: vec![
                vec![
                    None,
                    Some(8),
                    Some(12),
                    None,
                    None,
                    None,
                    None,
                    Some(6),
                    Some(3),
                    Some(1),
                    None,
                    None,
                ],
                vec![
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(5),
                    Some(3),
                    None,
                    None,
                    None,
                    None,
                    Some(2),
                ],
                vec![
                    Some(3),
                    None,
                    Some(9),
                    None,
                    Some(4),
                    Some(8),
                    None,
                    None,
                    None,
                    Some(7),
                    None,
                    Some(6),
                ],
                vec![
                    Some(9),
                    None,
                    None,
                    Some(1),
                    None,
                    Some(2),
                    None,
                    Some(8),
                    Some(6),
                    None,
                    None,
                    None,
                ],
                vec![
                    Some(8),
                    None,
                    None,
                    Some(11),
                    Some(7),
                    None,
                    None,
                    Some(9),
                    None,
                    Some(10),
                    None,
                    None,
                ],
                vec![
                    None,
                    Some(10),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(9),
                    Some(2),
                    Some(8),
                    None,
                ],
                vec![
                    None,
                    Some(11),
                    Some(6),
                    Some(12),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(10),
                    None,
                ],
                vec![
                    None,
                    None,
                    Some(1),
                    None,
                    Some(3),
                    None,
                    None,
                    Some(10),
                    Some(12),
                    None,
                    None,
                    Some(11),
                ],
                vec![
                    None,
                    None,
                    None,
                    Some(9),
                    Some(2),
                    None,
                    Some(8),
                    None,
                    Some(1),
                    None,
                    None,
                    Some(5),
                ],
                vec![
                    Some(7),
                    None,
                    Some(5),
                    None,
                    None,
                    None,
                    Some(4),
                    Some(3),
                    None,
                    Some(8),
                    None,
                    Some(1),
                ],
                vec![
                    Some(12),
                    None,
                    None,
                    None,
                    None,
                    Some(1),
                    Some(2),
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                vec![
                    None,
                    None,
                    Some(11),
                    Some(4),
                    Some(8),
                    None,
                    None,
                    None,
                    None,
                    Some(6),
                    Some(9),
                    None,
                ],
            ],
        }
    }

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
    fn test_solvable_12x12() {
        let mut solvable_test_instance = get_solvable_12x12_test_instance();
        assert!(!solvable_test_instance.is_solved());
        solvable_test_instance.solve();
        assert!(solvable_test_instance.is_solved());
        println!("{:?}", solvable_test_instance);
    }

    #[test]
    fn test_solvable_9x9() {
        let mut solvable_test_instance = get_solvable_9x9_test_instance();
        assert!(!solvable_test_instance.is_solved());
        solvable_test_instance.solve();
        assert!(solvable_test_instance.is_solved());
        println!("{:?}", solvable_test_instance);
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
