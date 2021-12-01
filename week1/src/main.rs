use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let filename = std::env::args().nth(1).expect("no filename given");
    println!("{}", filename);
    let input = File::open(filename)?;
    let reader = BufReader::new(input);
    let mut count = 0;
    let readings: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();
    for depth in 0..readings.len() - 1 {
        if readings[depth] < readings[depth + 1] {
            count += 1;
        }
    }
    println!("depth changes: {}", count);
    Ok(())
}
