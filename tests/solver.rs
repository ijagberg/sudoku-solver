#[cfg(test)]
mod tests {
    use sudoku::Sudoku;
    use sudoku_solver::{Backtrack, Solve};

    fn get_solvable_16x16_test_instance() -> Sudoku {
        let mut instance = Sudoku::new(16, 4, 4).unwrap();
        instance.populate_from_str(
            r#"_ _ _ _ _ 5 _ _ 6 _ 15 _ 11 _ 4 _
               _ _ 4 9 _ 14 10 _ 8 3 _ _ 13 _ _ _
               _ _ 14 6 8 16 13 _ _ 11 _ _ _ 15 _ _
               _ _ _ _ 1 _ _ _ _ 13 4 10 _ 8 3 6
               _ _ _ 8 _ 1 7 _ _ 4 2 3 _ _ 10 13
               _ _ _ _ 16 _ _ 14 13 10 _ _ _ _ _ _ 
               _ 14 5 2 13 6 4 _ _ 1 _ 9 _ 12 16 _
               _ _ _ _ _ _ 2 _ _ _ 11 _ _ 9 14 _
               _ _ _ _ 14 13 11 1 _ 8 12 _ 5 _ _ _
               _ _ _ 14 3 _ _ _ _ _ _ _ 7 _ 2 9
               _ _ _ _ _ _ _ _ _ _ _ 7 3 _ 12 15
               6 _ _ _ _ 15 _ _ 5 _ _ 13 _ 11 _ _
               _ _ _ _ 5 _ _ 15 _ _ _ 2 _ _ 1 14
               8 _ _ 16 _ _ _ _ _ _ _ _ 12 2 _ _
               _ 5 6 10 _ _ 16 _ 3 _ 1 4 _ 7 _ _
               _ 3 1 _ 9 _ _ _ 15 _ 7 _ 16 _ 5 _"#,
        );
        instance
    }

    fn get_unsolvable_16x16_test_instance() -> Sudoku {
        let mut instance = Sudoku::new(16, 4, 4).unwrap();
        instance.populate_from_str(
            r#"12 4 2 8 _ _ _ _ 7 1 _ _ _ _ _ _
            11 13 5 16 15 14 7 6 9 4 12 10 2 8 1 3
            _ _ _ 14 3 2 4 _ 8 _ 11 _ _ _ _ _  
            7 6 3 10 _ _ 16 _ 2 5 15 14 13 9 4 _ 
            _ 12 _ 4 _ _ 2 _ _ _ _ _ _ _ _ _
            14 8 _ 9 4 _ 6 7 _ _ _ _ 10 12 13 _
            2 7 13 6 8 9 10 11 _ _ _ _ _ _ 15 5
            3 16 _ _ _ _ _ _ _ _ _ _ _ _ _ 4
            8 _ _ _ _ _ _ _ _ _ _ _ _ _ 2 16
            10 _ _ _ _ _ _ _ 12 15 6 4 5 11 3 14
            _ 15 7 12 _ _ _ _ 5 3 _ 8 1 _ 9 10
            _ _ _ _ _ _ _ _ _ 10 _ _ 12 _ 8 _
            _ 11 16 15 5 10 13 2 _ 9 _ 12 3 7 14 8
            _ _ _ _ _ 3 _ 1 _ 2 13 16 15 _ _ _
            _ 10 6 _ 14 16 15 _ 3 7 _ _ _ 1 12 2
            _ _ _ _ _ _ 12 4 _ _ _ _ 11 10 16 14"#,
        );
        instance
    }

    fn get_solvable_12x12_test_instance() -> Sudoku {
        let mut instance = Sudoku::new(12, 4, 3).unwrap();
        instance.populate_from_str(
            r#"_ 8 12 _ _ _ _ 6 3 1  _ _
               _ _ _  _ _ 5 3 _ _ _  _ 2
               3 _ 9  _ 4 8 _ _ _ 7  _ 6
               9 _ _  1 _ 2 _ 8 6 _  _ _
               8 _ _ 11 7 _ _ 9 _ 10 _ _
               _ 10 _ _ _ _ _ _ 9 2 8 _
               _ 11 6 12 _ _ _ _ _ _ 10 _
               _ _ 1 _ 3 _ _ 10 12 _ _ 11
               _ _ _ 9 2 _ 8 _ 1 _ _ 5
               7 _ 5 _ _ _ 4 3 _ 8 _ 1
               12 _ _ _ _ 1 2 _ _ _ _ _ 
               _ _ 11 4 8 _ _ _ _ 6 9 _ "#,
        );
        instance
    }

    fn get_solvable_9x9_test_instance() -> Sudoku {
        let mut instance = Sudoku::new(9, 3, 3).unwrap();
        instance.populate_from_str(
            r#"_ _ 3 _ 2 _ 6 _ _
               9 _ _ 3 _ 5 _ _ 1
               _ _ 1 8 _ 6 4 _ _
               _ _ 8 1 _ 2 9 _ _
               7 _ _ _ _ _ _ _ 8
               _ _ 6 7 _ 8 2 _ _
               _ _ 2 6 _ 9 5 _ _
               8 _ _ 2 _ 3 _ _ 9
               _ _ 5 _ 1 _ 3 _ _"#,
        );
        instance
    }

    fn get_unsolvable_9x9_test_instance() -> Sudoku {
        let mut instance = Sudoku::new(9, 3, 3).unwrap();
        instance.populate_from_str(
            r#"_ _ 3 _ 2 _ 6 _ _
               9 _ _ 3 _ 5 _ _ 1
               _ _ 1 8 _ 6 4 _ _
               _ _ 8 1 _ 2 9 _ _
               7 _ _ _ _ _ _ _ 8
               _ _ 6 7 _ 8 2 _ _
               _ _ 2 6 _ 9 5 _ _
               8 _ _ 2 _ 3 3 _ 9
               _ _ 5 _ 1 _ _ _ _"#,
        );
        instance
    }

    fn get_solvable_6x6_test_instance() -> Sudoku {
        let mut instance = Sudoku::new(6, 3, 2).unwrap();
        instance.populate_from_str(
            r#"4 2 1 3 _ _
               _ _ 5 4 _ 1
               _ _ 2 6 _ _
               _ 6 4 _ _ _
               2 _ 6 1 _ 3
               1 5 3 2 4 _"#,
        );
        instance
    }

    // #[test]
    // fn test_solvable_16x16() {
    //     let unsolved = get_solvable_16x16_test_instance();
    //     assert_eq!(false, unsolved.is_solved());
    //     let solved = Backtrack::solve(unsolved);
    //     assert_eq!(true, solved.is_ok());
    // }

    #[test]
    fn test_solvable_12x12() {
        let unsolved = get_solvable_12x12_test_instance();
        assert_eq!(false, unsolved.is_solved());
        let solved = Backtrack::solve(unsolved);
        assert_eq!(true, solved.is_ok());
    }

    #[test]
    fn test_solvable_9x9() {
        let unsolved = get_solvable_9x9_test_instance();
        assert_eq!(false, unsolved.is_solved());
        let solved = Backtrack::solve(unsolved);
        assert_eq!(true, solved.is_ok());
    }

    #[test]
    fn test_unsolvable_9x9() {
        let unsolvable_test_instance = get_unsolvable_9x9_test_instance();
        assert_eq!(false, unsolvable_test_instance.is_solved());
        let solved = Backtrack::solve(unsolvable_test_instance);
        assert_eq!(true, solved.is_err());
    }

    #[test]
    fn test_solvable_6x6() {
        let unsolved = get_solvable_6x6_test_instance();
        assert_eq!(false, unsolved.is_solved());
        let solved = Backtrack::solve(unsolved);
        assert_eq!(true, solved.is_ok());
    }
}
