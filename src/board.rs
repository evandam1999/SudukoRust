pub struct Puzzle {
    pub board: Vec<u32>
}

impl Puzzle {
    pub fn new(board: Vec<u32>) -> Puzzle {
        Puzzle{
            board: board
        }
    }
}

impl std::ops::Index<usize> for Puzzle {
    type Output = u32;
    fn index(&self, i: usize) -> &u32 {
        return &self.board[i];        
    }
}


pub fn valid_row(row:usize, sol:&[u32]) -> bool {
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

pub fn valid_col(col:usize, sol:&[u32]) -> bool {
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

pub fn valid_grid(grid:usize, sol:&[u32]) -> bool {
    let mut agrid = Vec::new();
    //TODO: fix this to be a formula?  Might be faster this way
    match grid {
        0 => agrid = vec![sol[0], sol[1], sol[2], sol[9], sol[10], sol[11], sol[18], sol[19], sol[20]],
        1 => agrid = vec![sol[3], sol[4], sol[5], sol[12], sol[13], sol[14], sol[21], sol[22], sol[23]],
        2 => agrid = vec![sol[6], sol[7], sol[8], sol[15], sol[16], sol[17], sol[24], sol[25], sol[26]],
        3 => agrid = vec![sol[27], sol[28], sol[29], sol[36], sol[37], sol[38], sol[45], sol[46], sol[47]],
        4 => agrid = vec![sol[30], sol[31], sol[32], sol[39], sol[40], sol[41], sol[48], sol[49], sol[50]],
        5 => agrid = vec![sol[33], sol[34], sol[35], sol[42], sol[43], sol[44], sol[51], sol[52], sol[53]],
        6 => agrid = vec![sol[54], sol[55], sol[56], sol[63], sol[64], sol[65], sol[72], sol[73], sol[74]],
        7 => agrid = vec![sol[57], sol[58], sol[59], sol[66], sol[67], sol[68], sol[75], sol[76], sol[77]],
        8 => agrid = vec![sol[60], sol[61], sol[62], sol[69], sol[70], sol[71], sol[78], sol[79], sol[80]],
        _ => panic!()

    }    
    //check that there are values that are not between 0-9
    //since puzzle and sol are u32 negative numbers should not be an issue.
    let gt9: bool = agrid.iter().map(|&x| x > 9).any(|x| x == true);
    if gt9 {return false;}  
    //check that there are not duplicate values 1-9 (0 are ok)
    for n in 1..10 {
        let count = agrid.iter().filter(|&&x| x == n).count();
        if count > 1 {return false;}
    }
    return true;
}

pub fn get_row(index:usize) -> usize {
    return index/9;
}

pub fn get_col(index:usize) -> usize {
    return index%9;
}

pub fn get_grid(index:usize) -> usize {
    //Todo this could be optimized, maybe calcuate all three
    let row = get_row(index);
    let col = get_col(index);
    return (col / 3) + (row / 3) * 3;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn index_to_row() {
        assert_eq!(get_row(0),0);
        assert_eq!(get_row(8),0);
        assert_eq!(get_row(9),1);
        assert_eq!(get_row(80),8);
    }

    #[test]
    fn index_to_col() {
        assert_eq!(get_col(0),0);
        assert_eq!(get_col(8),8);
        assert_eq!(get_col(9),0);
        assert_eq!(get_row(80),8);
    }

    #[test]
    fn index_to_grid() {
        assert_eq!(get_grid(0),0);
        assert_eq!(get_grid(2),0);
        assert_eq!(get_grid(3),1);
        assert_eq!(get_grid(77),7);
        assert_eq!(get_grid(80),8);
    }

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
    fn is_valid_grid() {
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
        assert_eq!(valid_grid(0, &puzzle),true);
        assert_eq!(valid_grid(1, &puzzle),true);
        assert_eq!(valid_grid(6, &puzzle),false);
        assert_eq!(valid_grid(8, &puzzle),false);        
    }
}