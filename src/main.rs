use std::collections::HashSet;

const N: usize = 9;

struct SudokuBoard {
    /// Cells are represented by a linear vector of sets.
    /// Each set contains the possible values for a cell.
    /// - If the set for a cell is empty, the board, in its
    ///   current state, is unsolvable.
    /// - If the set is a singleton, we have found the solution
    ///   for the current cell.
    cells: Vec<HashSet<usize>>
}

impl SudokuBoard {
    fn from_str(descr: &str) -> Self {
        let mut v: Vec<HashSet<usize>> = Vec::with_capacity(N*N);
        for c in descr.chars() {
            match c {
                '.' => {
                    v.push((1..N+1).collect());
                }
                '1' ... '9' => {
                    // There ought to be a simpler way to create a singleton set.
                    let n = c.to_digit(10).unwrap() as usize;
                    let s = vec![n].into_iter().collect();
                    v.push(s);
                }
                _ => {
                    panic!("Not a valid string")
                }
            }
        }
        SudokuBoard { cells: v }
    }

    fn is_solved(&self) -> bool {
        self.cells.iter().all(|set| set.len() == 1)
    }

    fn get<'a>(&'a self, row: usize, col: usize) -> &'a HashSet<usize> {
        &self.cells[N*row + col]
    }

    fn candidates_for(&self, row: usize, col: usize) -> HashSet<usize> {
        let mut s = self.get(row, col).clone();

        // Row
        for i in 0..N {
            if i != col {
                let t = self.get(row, i);
                if t.len() == 1 {
                    for x in t {
                        s.remove(x);
                    }
                }
            }
        }

        // Col
        for i in 0..N {
            if i != row {
                let t = self.get(i, col);
                if t.len() == 1 {
                    for x in t {
                        s.remove(x);
                    }
                }
            }
        }

        // Group
        let gx = 3 * (row / 3);
        let gy = 3 * (col / 3);
        for i in gx..gx+3 {
            for j in gy..gy+3 {
                if i != row && j != col {
                    let t = self.get(i, j);
                    if t.len() == 1 {
                        for x in t {
                            s.remove(x);
                        }
                    }
                }
            }
        }

        return s;
    }


}

fn main() {
    let sb = SudokuBoard::from_str(
        "12..............................................................................."
    );

    println!("{:?}", sb.is_solved());
    println!("{:?}", sb.candidates_for(0, 0));
    println!("{:?}", sb.candidates_for(0, 1));
    println!("{:?}", sb.candidates_for(1, 0));
    println!("{:?}", sb.candidates_for(2, 2));
}
