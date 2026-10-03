mod generate;
const SIZE: i32 = 3;

#[derive(Debug, Clone, Copy)]
struct Sudoku {
    content: [i8; SIZE.pow(4) as usize],
    values: [i8; SIZE.pow(2) as usize]
}

fn main() {
    // let main_sudoku = Sudoku::random_generate();
    println!("passed {} tests",test());
}

fn test() -> i32 {
    let mut passes: i32 = 0;
    let test_all_pass: Sudoku = Sudoku { content: [1,2,3, 4,5,6, 7,8,9,
                                                    4,5,6, 7,8,9, 1,2,3,
                                                    7,8,9, 1,2,3, 4,5,6,

                                                    2,3,4, 5,6,7, 8,9,1,
                                                    5,6,7, 8,9,1, 2,3,4,
                                                    8,9,1, 2,3,4, 5,6,7,

                                                    3,4,5, 6,7,8, 9,1,2,
                                                    6,7,8, 9,1,2, 3,4,5,
                                                    9,1,2, 3,4,5, 6,7,8], values: [1,2,3,4,5,6,7,8,9] };
    let mut test_fail_column: Sudoku = test_all_pass.clone(); test_fail_column.content[27] = 1;
    let mut test_fail_line: Sudoku = test_all_pass.clone(); test_fail_line.content[8] = 1;
    let mut test_fail_squat: Sudoku = test_all_pass.clone(); test_fail_squat.content[19] = 2;

    if test_all_pass.validate() {passes+= 1}
    if !test_fail_column.validate() {passes+= 1}
    if !test_fail_line.validate() {passes+= 1}
    if !test_fail_squat.validate() {passes+= 1}
    return passes;
}