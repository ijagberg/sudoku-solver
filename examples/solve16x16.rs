use std::time::Instant;
use sudoku_solver::sudoku::Sudoku;

fn main() {
    let mut puzzle = Sudoku::new(16, 4, 4);
    puzzle.populate_from_str(
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

    println!("{:?}", puzzle);

    println!("Solving... this is gonna take a while (around an hour on my machine)");
    let now = Instant::now();
    match puzzle.solve() {
        Ok(_) => {
            println!("{:?}", puzzle);
        }
        Err(_) => println!("Couldn't solve instance"),
    }

    println!("Took {:?}", now.elapsed());
}
