use sudoku_solver::sudoku::Sudoku;

fn main() {
    let mut puzzle = Sudoku::new(9, 3, 3);
    puzzle.populate_from_str(
        r#"_ _ 3 _ 2 _ 6 _ _
               9 _ _ 3 _ 5 _ _ 1
               _ _ 1 8 _ 6 4 _ _
               _ _ 8 1 _ 2 9 _ _
               7 _ _ _ _ _ _ _ 8
               _ _ 6 7 _ 8 2 _ _
               _ _ 2 6 _ 9 5 _ _
               8 _ _ 2 _ 3 _ _ 9
               _ _ 5 _ 1 _ 3 _ _"#);

    println!("{:?}", puzzle);
}