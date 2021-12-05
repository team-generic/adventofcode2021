use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    let filename = std::env::args().nth(1).expect("no filename given");
    println!("{}", filename);
    let input = File::open(filename).expect("could not open file");
    let reader = BufReader::new(input);
    let mut lines_iter = reader.lines().map(|l| l.unwrap());

    let plays: Vec<isize> = lines_iter
        .next()
        .expect("no plays")
        .split(",")
        .map(|play| -> isize { play.parse::<isize>().unwrap() })
        .collect();

    let mut board = [[0; 5]; 5];
    let mut bingo_boards: Vec<Board> = Vec::new();
    while lines_iter.next() != None {
        for row in &mut board {
            *row = std::convert::TryFrom::<Vec<isize>>::try_from(
                lines_iter
                    .next()
                    .expect("no boards")
                    .split_whitespace()
                    .map(|play| -> isize { play.parse::<isize>().unwrap() })
                    .collect::<Vec<isize>>(),
            )
            .unwrap();
        }
        bingo_boards.push(Board::new(&board));
    }
    'outer: for play in plays {
        for bingo in &mut bingo_boards {
            if bingo.is_full() {
                continue;
            }
            if bingo.make_play(play) && bingo.is_full() {
                let mut sum: isize = 0;
                for elem in bingo as &Board {
                    if !elem.1 {
                        sum += elem.0;
                    }
                }
                println!("Solution: sum {} * play {} = {}", sum, play, sum * play);
            }
        }
    }
}
#[derive(Debug, Default, Clone)]
struct Board {
    board: [[isize; 5]; 5],
    plays: [[bool; 5]; 5],
}

impl Board {
    fn new(board: &[[isize; 5]; 5]) -> Board {
        Board {
            board: *board,
            plays: [[false; 5]; 5],
        }
    }

    fn is_full(&self) -> bool {
        let mut full: bool;
        for row in self.plays {
            full = true;
            for col in row {
                if col == false {
                    full = false;
                }
            }
            if full {
                return true;
            }
        }

        for col in 0..self.plays.len() {
            full = true;
            for row in 0..self.plays.len() {
                if self.plays[row][col] == false {
                    full = false;
                }
            }
            if full {
                return true;
            }
        }

        false
    }

    fn set(&mut self, row: usize, col: usize) {
        self.plays[row][col] = true;
    }

    fn find(&self, num: isize) -> Option<(usize, usize)> {
        for elem in self {
            if elem.0 == num {
                return Some((elem.2, elem.3));
            }
        }
        None
    }

    fn make_play(&mut self, num: isize) -> bool {
        match self.find(num) {
            Some(x) => {
                self.set(x.0, x.1);
                true
            }
            None => false,
        }
    }
    fn iter(&self) -> BoardIter {
        BoardIter {
            board: &self,
            row: 0,
            col: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = (isize, bool, usize, usize);
    type IntoIter = BoardIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
struct BoardIter<'a> {
    board: &'a Board,
    row: usize,
    col: usize,
}

impl<'a> Iterator for BoardIter<'a> {
    type Item = (isize, bool, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == 5 {
            return None;
        }
        let ret = Some((
            self.board.board[self.row][self.col],
            self.board.plays[self.row][self.col],
            self.row,
            self.col,
        ));
        if self.col == 4 {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        ret
    }
}
