use std::char;
use std::collections::BTreeSet;

const NSQRT: usize = 3;
const N: usize = NSQRT * NSQRT;
const NSQ: usize = N*N;

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
struct SudokuBoard(Vec<u8>);

impl SudokuBoard {
    /// Create a new sudoku board from a string.
    /// A non-zero digit stands for itself,
    /// a dot stands for a blank cell,
    /// anything else is an error.
    // TODO(vfoley): more error handling (length check)
    fn from_str(digits: &str) -> Self {
        let mut v = Vec::with_capacity(NSQ);
        for d in digits.chars() {
            match d {
                '.' => { v.push(0); }
                '1' ... '9' => {
                    let n = d.to_digit(10).unwrap() as u8;
                    v.push(n);
                }
                _ => { panic!("invalid digit ({:?}) in string", d); }
            }
        }
        SudokuBoard(v)
    }

    /// Convert a sudoku board to a rough string representation.
    fn to_str(&self) -> String {
        let mut s = String::with_capacity(N + NSQ);
        let mut i = 0;
        for n in self.0.iter() {
            if i % N == 0 {
                s.push('\n');
            }
            if *n == 0 {
                s.push('.');
            } else {
                s.push(char::from_digit(*n as u32, 10).unwrap());
            }
            i += 1;
        }
        s
    }

    /// A cell is solved if its content is non-zero.
    fn is_solved(&self, cell: usize) -> bool {
        self.0[cell] != 0
    }

    /// Return the list of candidates for a cell.
    /// The candidates are the digits from 1 to NSQ
    /// that are *not* solved in any of `cell`'s neighbors.
    fn candidates(&self, cell: usize) -> BTreeSet<u8> {
        let mut non_candidates = BTreeSet::new();
        for n in neighbors(cell) {
            let x = self.0[n];
            if x != 0 {
                non_candidates.insert(x);
            }
        }
        let mut candidates: BTreeSet<u8> = (1_u8 .. (N+1) as u8).collect();
        for nc in non_candidates {
            candidates.remove(&nc);
        }
        candidates
    }

    /// Solve a sudoku board by backtracking.
    /// Return true if the board is solved, false otherwise.
    fn solve(&mut self, cell: usize) -> bool {
        // If we reach the end of the board (NSQ'th cell),
        // we have solved the board, i.e., put digits into
        // cells without backtracking.
        if cell >= NSQ {
            return true;
        }

        // Skip over already-solved cells (initial solutions)
        if self.is_solved(cell) {
            return self.solve(cell + 1);
        }

        // Try the candidates in order.
        // Loop invariant: self.0[cell] = 0
        // coming into the loop and exiting the loop.
        for c in self.candidates(cell) {
            self.0[cell] = c;
            if self.solve(cell + 1) {
                return true;
            } else {
                self.0[cell] = 0;
            }
        }
        // No candidate satisfied the constraints, backtrack.
        return false;
    }
}

fn main() {
    let mut sb = SudokuBoard::from_str(".94...13..............76..2.8..1.....32.........2...6.....5.4.......8..7..63.4..8");
    println!("{}", sb.to_str());
    sb.solve(0);
    println!("{}", sb.to_str());
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
