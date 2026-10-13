use crate::{SIZE, Sudoku};
use rand;

impl Sudoku {
    pub fn new() -> Sudoku {
        return Sudoku { content: [0; SIZE.pow(4) as usize], values: [1,2,3,4,5,6,7,8,9] };
    }
    pub fn random_generate () -> Sudoku {
        let mut new_sudoku: Sudoku = Sudoku::new();

        for j in 0..3  as usize{
            for i in 0..9 as usize {
                while !new_sudoku.validate_square(((i % 3 + 9*(i / 3)) + j*30) as i32) || new_sudoku.content[(i % 3 + 9*(i / 3)) + j*30] == 0 {
                    new_sudoku.content[(i % 3 + 9*(i / 3)) + j*30] = rand::random_range(1..10);
                }
            }
        }

        new_sudoku.solve_sudoku();

        return new_sudoku;
    }
    pub fn validate (&self) -> bool {
        for sqr in 0..self.content.len() {
            if !self.validate_square(sqr as i32) {
                return false;
            }
        }
        return true;
    }
    pub fn validate_square (&self, square: i32) -> bool { // this function is an ABOMINATION, but it works :)
        
        if self.content[square as usize] == 0 {return true;} // we dont validate empty squares, as they conflict with other empty squares

        for i in -(SIZE.pow(2))..(SIZE.pow(2)) as i32 { // validate line
            if i == 0 || square+i < 0 {continue;}

            if (square+i)/(SIZE*SIZE) == square/(SIZE*SIZE) { // check if the row is the same, if not continue.
                if self.content[(square+i) as usize] == self.content[square as usize] {
                    //println!("row: sq: {} i: {}", square, i);
                    return false;
                }
            }
        }
        for i in 0..(SIZE*SIZE) {   // validate column
            if i == square / (SIZE*SIZE) {continue;}
            if self.content[(square % (SIZE*SIZE) + i * (SIZE*SIZE)) as usize] == self.content[square as usize] {
                //println!("col: sq: {}, i: {}", square, i);
                return false;
            } 
        }
        // validate squat
        let squad = (square / SIZE % SIZE) + (square / SIZE.pow(3) % SIZE)* SIZE;
        for x in 0..SIZE {
            for y in 0..SIZE {
                if square != (((squad / SIZE) * SIZE.pow(3) + (squad % SIZE)*SIZE) + (y * SIZE.pow(2)) + x) && self.content[square as usize] == self.content[(((squad / SIZE) * SIZE.pow(3) + (squad % SIZE)*SIZE) + (y * 9) + x) as usize] {
                    //println!("sqd: {} sqr: {} x: {}, y: {}, read: {}",squad, square, x, y,self.content[square as usize] == self.content[(((squad / SIZE) * SIZE.pow(3) + (squad % SIZE)*SIZE) + (y * 9) + x) as usize]);
                    return false;
                }
            }
        }
        return true;
    }
    pub fn solve_square (&mut self, square: i32) -> bool {
        let old_square_val = self.content[square as usize].clone();
        let mut n_possibilities = 0;

        for i in 1..10 as i8 {
            self.content[square as usize] = i;
            if self.validate_square(square) {
                n_possibilities += 1;
            }
        }

        if !n_possibilities == 1 {
            self.content[square as usize] = old_square_val;
        }
        return n_possibilities == 1;
    }
    pub fn solve_sudoku (&mut self) -> bool {
        let mut fails = 1; //set to 1 at start for the while loop to start at all
        let mut last_fails = 0;

        while fails < 0 {
            
            last_fails = fails;
            fails = 0;

            for i in 0..SIZE.pow(4) {
                if self.content[i as usize] == 0 {
                    if !self.solve_square(i) {fails += 1;}
                }
            }
        }
        return true;
    }
}