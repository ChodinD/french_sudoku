use crate::{SIZE, Sudoku};

impl Sudoku {
    pub fn new() -> Sudoku {
        return Sudoku { content: [0; SIZE.pow(4) as usize], values: [1,2,3,4,5,6,7,8,9] };
    }
    pub fn random_generate () -> Sudoku {
        let mut new_sudoku: Sudoku = Sudoku::new();

        for j in 0..9 as usize{

            for i in 0..9 as usize {
                new_sudoku.content[i+j*9] = i as i8+1;
            }
        }        
        return new_sudoku;
    }
    pub fn validate (&self) -> bool {
        let mut is_valid: bool = true;
        for sqr in 0..self.content.len() {
            if !self.validate_square(sqr as i32) {
                return false;
                break;
            }
        }
        return is_valid;
    }
    pub fn validate_square (&self, square: i32) -> bool { // this function is an ABOMINATION, but it works :)
        
        for i in -(SIZE.pow(2))..(SIZE.pow(2)) as i32 { // validate line
            if i == 0 || square+i < 0 {continue;}

            if (square+i)/(SIZE*SIZE) == square/(SIZE*SIZE) { // check if the row is the same, if not continue.
                if self.content[(square+i) as usize] == self.content[square as usize] {
                    println!("row: sq: {} i: {}", square, i);
                    return false;
                }
            }
        }
        for i in 0..(SIZE*SIZE) {   // validate column
            if i == square / (SIZE*SIZE) {continue;}
            if self.content[(square % (SIZE*SIZE) + i * (SIZE*SIZE)) as usize] == self.content[square as usize] {
                println!("col: sq: {}, i: {}", square, i);
                return false;
            } 
        }
        // validate squat
        let squad = (square / SIZE % SIZE) + (square / SIZE.pow(3) % SIZE)* SIZE;
        for x in 0..SIZE {
            for y in 0..SIZE {
                if square != (((squad / SIZE) * SIZE.pow(3) + (squad % SIZE)*SIZE) + (y * SIZE.pow(2)) + x) && self.content[square as usize] == self.content[(((squad / SIZE) * SIZE.pow(3) + (squad % SIZE)*SIZE) + (y * 9) + x) as usize] {
                    /*Debug line: */println!("sqd: {} sqr: {} x: {}, y: {}, read: {}",squad, square, x, y,self.content[square as usize] == self.content[(((squad / SIZE) * SIZE.pow(3) + (squad % SIZE)*SIZE) + (y * 9) + x) as usize]);
                    return false;
                }
            }
        }
        return true;
    }
    pub fn sovle_square (self, square: i32) -> Sudoku {
        return Sudoku::new();
    }
}