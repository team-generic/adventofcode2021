use bitvec::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let filename = std::env::args().nth(1).expect("no filename given");
    println!("{}", filename);
    let input = File::open(filename)?;
    let reader = BufReader::new(input);

    let sz: usize = 0b101;
    for i in 0..3 {
        println!("{:b}", sz.overflowing_shr(i).0 & 1);
    }
    let readings: Vec<Vec<usize>> = reader
        .lines()
        .map(|line| -> Vec<usize> {
            line.unwrap()
                .chars()
                .map(|letter| -> usize {
                    match letter {
                        '1' => 1,
                        '0' => 0,
                        _ => panic!("not looking good"),
                    }
                })
                .collect()
        })
        .collect();
    let n: usize = readings.len();
    let result: Vec<usize> = match readings
        .into_iter()
        .reduce(|accum, item| accum.iter().zip(item.iter()).map(|a| a.0 + a.1).collect())
    {
        Some(x) => x,
        None => panic!(),
    };

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    println!("{} items: {:?}", n, result);
    for res in result {
        if 2 * res < n {
            gamma = gamma << 1;
            epsilon = epsilon << 1 | 1;
        } else {
            gamma = gamma << 1 | 1;
            epsilon = epsilon << 1;
        }
    }
    println!("gamma: 'b{:b}, epsilon: 'b{:b}", gamma, epsilon);
    println!("Result: {}", gamma * epsilon);

    Ok(())
}
