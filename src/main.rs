use std::{time};
use std::time::{Duration, Instant};
mod board;

fn main() {        
    let now = Instant::now();
    let mut guesses = 0;

    //let puzzle = ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4";
    let puzzle = "8..........36......7..9.2...5...7.......457.....1...3...1....68..85...1..9....4..";
    let check_solution = "812753649943682175675491283154237896369845721287169534521974368438526917796318452";

    let puzzle = puzzle.replace(".", "0");
    //let puzzle:Vec<u32> = puzzle.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let puzzle = board::Puzzle::new(puzzle.chars().map(|c| c.to_digit(10).unwrap()).collect());

    let mut solution = puzzle.board.clone();

    let mut curidx: usize = 0;
    let mut previdx:Vec<(usize,u32)> = Vec::new();
    let mut guess_from = 1;
    'outer: loop {
        if curidx == 81 {
            break 'outer;
        }
        //println!("index {}", curidx);        
        if puzzle[curidx] != 0 {
            curidx = curidx+1;
            continue;
        }
        else { //curidx == 0
            //Lets guess
            'guess: for n in guess_from..=9 { //n is your number you are putting in the solution
                guesses = guesses+1;
                //println!("guessing {}", n);
                //thread::sleep(sleeptime);
                solution[curidx] = n;
                //println!("{:?}", solution);
                //is this guess any good?
                let curidx_valid_row = board::valid_row(board::get_row(curidx), &solution);
                let curidx_valid_col = board::valid_col(board::get_col(curidx), &solution);                
                if  !curidx_valid_row || !curidx_valid_col || !board::valid_grid(board::get_grid(curidx), &solution) {
                    //guess is NOT valid                        
                    if n == 9 {
                        guess_from = 10;
                        while guess_from > 9 {
                            solution[curidx] = 0;
                            let prev = previdx.pop();
                            curidx = prev.unwrap().0 as usize;
                            guess_from = prev.unwrap().1 as u32 + 1;
                            //println!("new curidx:{}, guessfrom:{}", curidx, guess_from);
                        }
                        break 'guess;
                    }
                    else {                            
                        continue 'guess;
                    }
                } else {
                    //guess is valid
                    previdx.push((curidx,n));
                    curidx = curidx+1;
                    guess_from = 1;
                    //println!("good guess moving on");
                    break 'guess;
                }   
            }
        }        
    }
    let correct:bool = solution.iter().map(|&x| x.to_string()).collect::<String>() == check_solution;

    if correct {
        println!("{:?}", solution);        
        println!("guesses: {}", guesses);
        println!("time: {}ms", now.elapsed().as_millis());
    }
    else {
        println!("Something went wrong");
    }
}