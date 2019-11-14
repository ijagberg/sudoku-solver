#[derive(Clone, PartialEq)]
pub struct Sudoku {
    size: usize,
    sec_width: usize,
    sec_height: usize,
    grid: Vec<Vec<Option<u32>>>,
}

impl Sudoku {
    /// Create a new instance of a Sudoku puzzle
    ///
    /// `size` is the number of unique values that should
    /// occupy each row, column, and section. A normal Sudoku puzzle
    /// has `size=9`
    ///
    /// `sec_width` is the width of the sections in the puzzle.
    /// `sec_height` is the height of the sections in the puzzle.
    ///
    /// # Panics
    /// If `sec_width * sec_height != size`
    pub fn new(size: usize, sec_width: usize, sec_height: usize) -> Self {
        if sec_width * sec_height != size {
            panic!("size must be equal to sec_width * sec_height");
        }

        Self {
            size,
            sec_width,
            sec_height,
            grid: vec![vec![None; size]; size],
        }
    }

    pub fn populate_from_str(&mut self, s: &str) {
        let lines: Vec<&str> = s.split('\n').collect();
        assert_eq!(self.size, lines.len());
        for (row, cols) in lines
            .iter()
            .enumerate()
            .map(|(row, line)| (row, line.split_whitespace().collect::<Vec<_>>()))
        {
            assert_eq!(self.size, cols.len());
            for (col, &value) in cols.iter().enumerate() {
                let entry = match value {
                    "_" => None,
                    word => match word.parse::<u32>() {
                        Ok(v) if v > 0 && v as usize <= self.size => Some(v),
                        Ok(v) => panic!(
                            "{} is not a valid value for a Sudoku with size {}",
                            v, self.size
                        ),
                        Err(_) => panic!("Invalid format of populating string"),
                    },
                };
                self.grid[row][col] = entry;
            }
        }
    }

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
