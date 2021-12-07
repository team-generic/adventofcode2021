use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = std::env::args().nth(1).expect("no filename given");
    println!("{}", filename);
    let sims: usize = std::env::args()
        .nth(2)
        .expect("no simulations given")
        .parse::<usize>()
        .unwrap();
    let input = File::open(filename).expect("could not open file");
    let reader = BufReader::new(input);
    let lines_iter = reader.lines().map(|l| l.unwrap());
    let mut fish_arr: [isize; 9] = [0; 9];
    let mut fish_start: Vec<usize> = Vec::new();
    for line in lines_iter {
        fish_start = line
            .split(',')
            .map(|l| -> usize { l.parse::<usize>().unwrap() })
            .collect();
    }
    for fish in fish_start {
        fish_arr[fish] += 1;
    }

    let mut fish = FishState { state: fish_arr };
    for _ in 0..sims {
        fish = fish.next_state();
    }
    println!("There are {} fish after {} days.", fish.num_fish(), sims);
}
struct FishState {
    state: [isize; 9],
}

impl FishState {
    fn next_state(&self) -> FishState {
        FishState {
            state: [
                self.state[1],
                self.state[2],
                self.state[3],
                self.state[4],
                self.state[5],
                self.state[6],
                self.state[7] + self.state[0],
                self.state[8],
                self.state[0],
            ],
        }
    }
    fn num_fish(&self) -> isize {
        self.state.iter().sum()
    }
}
