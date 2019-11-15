use sudoku_solver::sudoku::Sudoku;

fn main() {
    // let mut puzzle = Sudoku::new(6, 3, 2);
    let mut puzzle = Sudoku::new(6, 2, 3);
    puzzle.populate_from_str(
        r#"4 2 1 3 _ _
           _ _ 5 4 _ 1
           _ _ 2 6 _ _
           _ 6 4 _ _ _
           2 _ 6 1 _ 3
           1 5 3 2 4 _"#,
    );

    println!("{:?}", puzzle);
}
