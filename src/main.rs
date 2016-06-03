use std::collections::BTreeSet;

const N: usize = 9;

fn row(linear_index: usize) -> usize {
    linear_index / N
}

fn col(linear_index: usize) -> usize {
    linear_index % N
}

struct SudokuBoard {
    /// Cells are represented by a linear vector of sets.
    /// Each set contains the possible values for a cell.
    /// - If the set for a cell is empty, the board, in its
    ///   current state, is unsolvable.
    /// - If the set is a singleton, we have found the solution
    ///   for the current cell.
    cells: Vec<BTreeSet<usize>>
}

impl SudokuBoard {
    fn from_str(descr: &str) -> Self {
        let mut v: Vec<BTreeSet<usize>> = Vec::with_capacity(N*N);
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
        self.cells.iter().all(|candidates| candidates.len() == 1)
    }

    fn cell_is_solved(&self, idx: usize) -> bool {
        self.cells[idx].len() == 1
    }


    /// Return the  indices of the neighbors of a cell, i.e.,
    /// the indices of the cells in the same row, same column and same
    /// square group.
    fn neighbors(&self, idx: usize) -> BTreeSet<usize> {
        let mut neighbor_indices = BTreeSet::new();

        // Row & column
        for i in 0..N {
            let row_idx = row(idx) + i;
            if row_idx != idx {
                neighbor_indices.insert(row_idx);
            }

            let col_idx = (N*i) + col(idx);
            if col_idx != idx {
                neighbor_indices.insert(col_idx);
            }
        }

        // Group
        let x = 3 * (row(idx) / 3);
        let y = 3 * (col(idx) / 3);
        for i in x .. x+3 {
            for j in y .. y+3 {
                let group_idx = (N*i) + j;
                if group_idx != idx {
                    neighbor_indices.insert(group_idx);
                }
            }
        }

        neighbor_indices
    }

    /*
    fn update_candidates(&mut self, row: usize, col: usize) {
        if self.cell_is_solved(linear_index(row, col)) {
            return;
        }

        // Put non-candidates in a vector, because we cannot
        // do self.cells[idx].remove(x) since self.cells is
        // already borrowed.
        let mut non_candidates: Vec<usize> = Vec::new();
        for n in self.neighbors(row, col) {
            if self.cell_is_solved(n) {
                for x in self.cells[n].iter() {
                    non_candidates.push(*x);
                }
            }
        }
        let idx = linear_index(row, col);
        for nc in non_candidates {
            self.cells[idx].remove(&nc);
        }
    }
    */
    fn print(&self) {
        for i in 0..N*N {
            if self.cell_is_solved(i) {
                for c in self.cells[i].iter() {
                    print!("{}", c);
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }
}


fn main() {
    let mut sb = SudokuBoard::from_str(
        "............942.8.16.....29........89.6.....14..25......4.......2...8.9..5....7.."
    );
}
