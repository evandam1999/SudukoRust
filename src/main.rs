use std::{time};

fn main() {    
    let sleeptime = time::Duration::from_millis(250 as u64);
    let mut guesses = 0;

    let puzzle = ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4";

    let puzzle = puzzle.replace(".", "0");
    let puzzle:Vec<u32> = puzzle.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut solution = puzzle.clone();

    let mut curidx = 0;
    let mut previdx:Vec<(usize,u32)> = Vec::new();
    let mut guess_from = 1;
    'outer: loop {
        if curidx == 80 {
            break 'outer;
        }
        println!("index {}", curidx);
        //thread::sleep(sleeptime);
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
                for k in 0..=8 { //these are the rows/columns and squares that you are checking
                    //Todo this is overkill
                    if  !valid_row(k as usize, &solution) || 
                        !valid_col(k as usize, &solution) ||
                        !valid_sqr(k as usize, &solution) {
                            //guess is NO good
                            //println!("{} is no good: blk: {}", n, k);
                            if n == 9 {
                                guess_from = 10;
                                while guess_from > 9 {
                                    solution[curidx] = 0;
                                    let prev = previdx.pop();
                                    curidx = prev.unwrap().0 as usize;
                                    guess_from = prev.unwrap().1 as u32 + 1;
                                    println!("new curidx:{}, guessfrom:{}", curidx, guess_from);
                                }
                                break 'guess;
                            }
                            else {                            
                                continue 'guess;
                            }
                         }
                }
                //guess is good
                // if curidx == 80 {
                //     break 'outer;
                // }
                previdx.push((curidx,n));
                curidx = curidx+1;
                guess_from = 1;
                println!("good guess moving on");
                break 'guess;
            }
        }        
    }
    println!("{:?}", solution);
    println!("guesses: {}", guesses);
}

fn valid_row(row:usize, sol:&[u32]) -> bool {
    let a = row*9;    
    let arow = &sol[a..a+9];
    //check that there are values that are not between 0-9
    //since puzzle and sol are u32 negative numbers should not be an issue.
    let gt9: bool = arow.iter().map(|&x| x > 9).any(|x| x == true);
    if gt9 {return false;}  
    //check that there are not duplicate values 1-9 (0 are ok)
    for n in 1..10 {
        let count = arow.iter().filter(|&&x| x == n).count();
        if count > 1 {return false;}
    }
    return true;
}

fn valid_col(col:usize, sol:&[u32]) -> bool {
    let mut acol = Vec::new();
    for n in 0..9 {
        acol.push(sol[col+(9*n)])
    }    
    //check that there are values that are not between 0-9
    //since puzzle and sol are u32 negative numbers should not be an issue.
    let gt9: bool = acol.iter().map(|&x| x > 9).any(|x| x == true);
    if gt9 {return false;}  
    //check that there are not duplicate values 1-9 (0 are ok)
    for n in 1..10 {
        let count = acol.iter().filter(|&&x| x == n).count();
        if count > 1 {return false;}
    }
    return true;
}

fn valid_sqr(sqr:usize, sol:&[u32]) -> bool {
    let mut asqr = Vec::new();
    //TODO: fix this to be a formula?  Might be faster this way
    match sqr {
        0 => asqr = vec![sol[0], sol[1], sol[2], sol[9], sol[10], sol[11], sol[18], sol[19], sol[20]],
        1 => asqr = vec![sol[3], sol[4], sol[5], sol[12], sol[13], sol[14], sol[21], sol[22], sol[23]],
        2 => asqr = vec![sol[6], sol[7], sol[8], sol[15], sol[16], sol[17], sol[24], sol[25], sol[26]],
        3 => asqr = vec![sol[27], sol[28], sol[29], sol[36], sol[37], sol[38], sol[45], sol[46], sol[47]],
        4 => asqr = vec![sol[30], sol[31], sol[32], sol[39], sol[40], sol[41], sol[48], sol[49], sol[50]],
        5 => asqr = vec![sol[33], sol[34], sol[35], sol[42], sol[43], sol[44], sol[51], sol[52], sol[53]],
        6 => asqr = vec![sol[54], sol[55], sol[56], sol[63], sol[64], sol[65], sol[72], sol[73], sol[74]],
        7 => asqr = vec![sol[57], sol[58], sol[59], sol[66], sol[67], sol[68], sol[75], sol[76], sol[77]],
        8 => asqr = vec![sol[60], sol[61], sol[62], sol[69], sol[70], sol[71], sol[78], sol[79], sol[80]],
        _ => panic!()

    }    
    //check that there are values that are not between 0-9
    //since puzzle and sol are u32 negative numbers should not be an issue.
    let gt9: bool = asqr.iter().map(|&x| x > 9).any(|x| x == true);
    if gt9 {return false;}  
    //check that there are not duplicate values 1-9 (0 are ok)
    for n in 1..10 {
        let count = asqr.iter().filter(|&&x| x == n).count();
        if count > 1 {return false;}
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]    
    fn is_valid_row() {
        let puzzle: [u32; 81] = 
        [1,2,3,4,5,6,7,8,9, //valid row
        1,2,3,4,5,10,7,8,9, // value > 9
        1,2,3,4,5,6,7,9,9,  // duplicate
        0,0,0,0,0,0,0,0,0,  // all zeros
        0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0];                                  
        assert_eq!(valid_row(0, &puzzle),true);
        assert_eq!(valid_row(1, &puzzle),false);
        assert_eq!(valid_row(2, &puzzle),false);
        assert_eq!(valid_row(3, &puzzle),true);        
    }

    #[test]
    fn is_valid_col() {
        let puzzle: [u32; 81] = 
        [1,1,1,0,0,0,0,0,0,
         2,2,2,0,0,0,0,0,0,
         3,10,3,0,0,0,0,0,0,
         4,4,3,0,0,0,0,0,0,
         5,5,5,0,0,0,0,0,0,
         6,6,6,0,0,0,0,0,0,
         7,7,7,0,0,0,0,0,0,
         8,8,8,0,0,0,0,0,0,
         9,9,9,0,0,0,0,0,0];
        assert_eq!(valid_col(0, &puzzle),true);
        assert_eq!(valid_col(1, &puzzle),false);
        assert_eq!(valid_col(2, &puzzle),false);        
    }

    #[test]
    fn is_valid_sqr() {
        let puzzle: [u32; 81] = 
        [1,2,3,0,0,0,0,0,0,
         4,5,6,0,0,0,0,0,0,
         7,8,9,0,0,0,0,0,0,
         4,4,3,0,0,0,0,0,0,
         5,5,5,0,0,0,0,0,0,
         6,6,6,0,0,0,0,0,0,
         7,7,7,0,0,0,1,1,0,
         8,8,8,0,0,0,0,0,0,
         9,9,9,0,0,0,0,0,0];
        assert_eq!(valid_sqr(0, &puzzle),true);
        assert_eq!(valid_sqr(1, &puzzle),true);
        assert_eq!(valid_sqr(6, &puzzle),false);
        assert_eq!(valid_sqr(8, &puzzle),false);        
    }
}
