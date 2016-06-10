use std::char;
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

    // Neighbors on row
    for col in 0..N {
        all_neighbors.insert((N * row(cell)) + col);
    }

    // Neighbors on column
    for row in 0..N {
        all_neighbors.insert((N * row) + col(cell));
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

    fn cell_solved(&self, cell: usize) -> bool {
        self.0[cell].len() == 1
    }

    fn cell_solvable(&self, cell: usize) -> bool {
        self.0[cell].len() != 0
    }

    fn solved(&self) -> bool {
        (0 .. NSQ).all(|cell| self.cell_solved(cell))
    }

    fn solvable(&self) -> bool {
        (0 .. NSQ).all(|cell| self.cell_solvable(cell))
    }

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


    fn solve(&self, cell: usize) -> Option<Self> {
        if cell >= NSQ {
            return Some(SudokuBoard(self.0.clone()));
        }

        if self.cell_solved(cell) {
            return self.solve(cell + 1);
        }

        let mut newboard = self.propagate();

        if newboard.solved() {
            return Some(newboard);
        }

        if !newboard.solvable() {
            return None;
        }

        let cell_candidates = newboard.0[cell].clone();
        for c in cell_candidates.iter() {
            newboard.0[cell].clear();
            newboard.0[cell].insert(*c);
            match newboard.solve(cell + 1) {
                Some(solved_board) => { return Some(solved_board); }
                None => { }
            }
        }

        return None;
    }

    fn debug(&self) {
        for i in 0 .. NSQ {
            println!("[{:?}]", self.0[i]);
        }
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
        match sb.solve(0) {
            Some(_) => { println!("OK"); }
            None => { println!("No solution"); }
        }
    }
}

/*
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

#[test]
fn test_is_solved() {
    let sb = SudokuBoard(vec![0, 1]);
    assert!(!sb.is_solved(0));
    assert!(sb.is_solved(1));
}

#[test]
fn test_neighbors() {
    assert!(!neighbors(0).contains(&0));
    // Neighbors of 0 on the same row
    assert!(neighbors(0).contains(&1));
    assert!(neighbors(0).contains(&2));
    assert!(neighbors(0).contains(&3));
    assert!(neighbors(0).contains(&4));
    assert!(neighbors(0).contains(&5));
    assert!(neighbors(0).contains(&6));
    assert!(neighbors(0).contains(&7));
    assert!(neighbors(0).contains(&8));
    // Neighbors of 0 on the same col
    assert!(neighbors(0).contains(&9));
    assert!(neighbors(0).contains(&18));
    assert!(neighbors(0).contains(&27));
    assert!(neighbors(0).contains(&36));
    assert!(neighbors(0).contains(&45));
    assert!(neighbors(0).contains(&54));
    assert!(neighbors(0).contains(&63));
    assert!(neighbors(0).contains(&72));
    // Neighbors of 0 in the same group
    assert!(neighbors(0).contains(&1));
    assert!(neighbors(0).contains(&2));
    assert!(neighbors(0).contains(&9));
    assert!(neighbors(0).contains(&10));
    assert!(neighbors(0).contains(&11));
    assert!(neighbors(0).contains(&18));
    assert!(neighbors(0).contains(&19));
    assert!(neighbors(0).contains(&20));
}

#[test]
fn test_candidates() {
    let sb = SudokuBoard::from_str(".2......34..........6...................................................5........");
    assert!(!sb.candidates(0).contains(&2));
    assert!(!sb.candidates(0).contains(&3));
    assert!(!sb.candidates(0).contains(&4));
    assert!(!sb.candidates(0).contains(&5));
    assert!(!sb.candidates(0).contains(&6));
    assert!(sb.candidates(0).contains(&1));
    assert!(sb.candidates(0).contains(&7));
    assert!(sb.candidates(0).contains(&8));
    assert!(sb.candidates(0).contains(&9));
}
*/
