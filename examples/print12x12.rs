use sudoku_solver::sudoku::Sudoku;

fn main() {
    let mut puzzle = Sudoku::new(12, 4, 3);
    puzzle.populate_from_str(
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

    println!("{:?}", puzzle);
}
