use std::collections::BTreeSet;
use std::io;
use std::process;

const NSQRT: usize = 3;
const N: usize = NSQRT * NSQRT;
const NSQ: usize = N*N;

fn error(msg: String) -> ! {
    println!("error: {}", msg);
    process::exit(1);
}

/// Return the 0-based row index of `cell`.
fn row(cell: usize) -> usize {
    cell / N
}

/// Return the 0-based row index of `cell`.
fn col(cell: usize) -> usize {
    cell % N
}

/// Return the 0-based index of the upper-left cell of `cell`'s group.
fn group(cell: usize) -> usize {
    let r = row(cell);
    let c = col(cell);
    (N * (r - r % NSQRT)) + (c - c % NSQRT)
}

/// Return the neighbors of `cell`:
/// - The cells on the same row;
/// - The cells on the same column;
/// - The same in the same group.
/// Note: `cell` is not a neighbor of itself.
fn neighbors(cell: usize) -> BTreeSet<usize> {
    let mut all_neighbors: BTreeSet<usize> = BTreeSet::new();

    // Neighbors in row and column
    for i in 0..N {
        all_neighbors.insert((N * row(cell)) + i);
        all_neighbors.insert((N * i) + col(cell));
    }

    // Neighbors in group
    let leader = group(cell);
    for r in row(leader) .. row(leader) + NSQRT {
        for c in col(leader) .. col(leader) + NSQRT {
            all_neighbors.insert(N * r + c);
        }
    }

    all_neighbors.remove(&cell);
    return all_neighbors;
}

/// A sudoku board is represented by a vector of bytes.
#[derive(Debug)]
struct SudokuBoard(Vec<BTreeSet<usize>>);

impl SudokuBoard {
    /// Create a new sudoku board from a string.
    /// A non-zero digit stands for itself,
    /// a dot stands for a blank cell,
    /// anything else is an error.
    fn from_str(digits: &str) -> Self {
        if digits.len() != NSQ {
            error(format!("invalid puzzle length; expected {}, got {}",
                          NSQ, digits.len()));
        }
        let mut v = Vec::with_capacity(NSQ);
        for d in digits.chars() {
            match d {
                '.' => {
                    let candidates: BTreeSet<usize> = (1 .. N+1).collect();
                    v.push(candidates);
                }
                '1' ... '9' => {
                    let mut singleton = BTreeSet::new();
                    singleton.insert(d.to_digit(10).unwrap() as usize);
                    v.push(singleton);
                }
                _ => { error(format!("invalid digit ({:?}) in string", d)); }
            }
        }
        SudokuBoard(v)
    }

    /// A cell is solved if its set of candidates is a singleton.
    fn cell_solved(&self, cell: usize) -> bool {
        self.0[cell].len() == 1
    }

    /// A cell is solvable if it has at least one candidate.
    fn cell_solvable(&self, cell: usize) -> bool {
        self.0[cell].len() != 0
    }

    /// The board is solved if all cells are solved.
    fn solved(&self) -> bool {
        (0 .. NSQ).all(|cell| self.cell_solved(cell))
    }

    /// The board is solvable is all cells are solvable.
    fn solvable(&self) -> bool {
        (0 .. NSQ).all(|cell| self.cell_solvable(cell))
    }

    /// The non-candidates of cells are the solved values in
    /// the cell's neighbors.
    fn non_candidates(&self, cell: usize) -> BTreeSet<usize> {
        let mut set: BTreeSet<usize> = BTreeSet::new();
        for n in neighbors(cell) {
            if self.cell_solved(n) {
                for x in self.0[n].iter() {
                    set.insert(*x);
                }
            }
        }
        set
    }

    /// Remove non-candidates from the cells of the board
    /// until a fixed point is reached, i.e., no more non-
    /// candidates can be removed anymore.
    fn propagate(&self) -> Self {
        let mut output = SudokuBoard(self.0.clone());
        loop {
            let mut candidates_changed = false;
            for i in 0 .. NSQ {
                for nc in output.non_candidates(i) {
                    let q = output.0[i].remove(&nc);
                    candidates_changed = q || candidates_changed;
                }
            }
            if !candidates_changed {
                break;
            }
        }
        return output;
    }

    /// Find the index of the unsolved cell with the
    /// fewest number of candidates.  Helps to speed
    /// up the solving process by making the search tree
    /// narrower.
    fn most_promising(&self) -> Option<usize> {
        let mut min_len = N;
        let mut min_index = NSQ;

        for i in 0 .. NSQ {
            if self.cell_solved(i) {
                continue;
            }
            let len = self.0[i].len();
            if len < min_len {
                min_index = i;
                min_len = len;
            }
        }

        if min_index == NSQ {
            None
        } else {
            Some(min_index)
        }
    }

    /// Solve the Sudoku board:
    /// 1. Propagate the set constraints
    /// 2a. If the board is solved, terminate.
    /// 2b. If the board is unsolvable, backtrack.
    /// 3. Pick the most promising cell and brute-force it.
    fn solve(&self) -> Option<Self> {
        let mut newboard = self.propagate();

        if newboard.solved() {
            return Some(newboard);
        }

        if !newboard.solvable() {
            return None;
        }

        if let Some(cell) = newboard.most_promising() {
            let cell_candidates = newboard.0[cell].clone();
            for c in cell_candidates.iter() {
                newboard.0[cell].clear();
                newboard.0[cell].insert(*c);
                match newboard.solve() {
                    Some(solved_board) => { return Some(solved_board); }
                    None => { }
                }
            }
        }

        return None;
    }

    /// Convert the board to a linear textual representation.
    fn to_str(&self) -> String {
        let mut output = String::with_capacity(NSQ);
        for i in 0 .. NSQ {
            if self.cell_solved(i) {
                for c in self.0[i].iter() {
                    output.push_str(&format!("{}", c));
                }
            } else {
                output.push('.');
            }
        }
        output
    }
}



fn main() {
    let stdin = io::stdin();
    let mut buf = String::with_capacity(NSQ);

    loop {
        buf.clear();
        match stdin.read_line(&mut buf) {
            Err(e) => { error(format!("I/O error, {:?}", e)); }
            Ok(0) => { return; }
            Ok(_) => { /* pass through */ }
        }
        let sb = SudokuBoard::from_str(&buf.trim());
        match sb.solve() {
            Some(solution) => { println!("{}", solution.to_str()); }
            None => { println!("No solution"); }
        }
    }
}

#[test]
fn test_row_col() {
    assert_eq!(row(11), 1);
    assert_eq!(col(11), 2);
}

#[test]
fn test_group() {
    assert_eq!(group(0), 0);
    assert_eq!(group(1), 0);
    assert_eq!(group(2), 0);
    assert_eq!(group(9), 0);
    assert_eq!(group(10), 0);
    assert_eq!(group(11), 0);
    assert_eq!(group(18), 0);
    assert_eq!(group(19), 0);
    assert_eq!(group(20), 0);
    assert_eq!(group(60), 60);
    assert_eq!(group(61), 60);
    assert_eq!(group(62), 60);
    assert_eq!(group(69), 60);
    assert_eq!(group(70), 60);
    assert_eq!(group(71), 60);
    assert_eq!(group(78), 60);
    assert_eq!(group(79), 60);
    assert_eq!(group(80), 60);
}
