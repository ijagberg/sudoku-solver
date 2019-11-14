#[cfg(test)]
mod tests {
    use sudoku_solver::sudoku::Sudoku;
    fn get_solvable_12x12_test_instance() -> Sudoku {
        let mut instance = Sudoku::new(12, 4, 3);
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
        let mut instance = Sudoku::new(9, 3, 3);
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
        let mut instance = Sudoku::new(9, 3, 3);
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
        let mut instance = Sudoku::new(6, 3, 2);
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
