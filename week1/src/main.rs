use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let filename = std::env::args().nth(1).expect("no filename given");
    println!("{}", filename);
    let window_size = match std::env::args().nth(2) {
        Some(x) => x.parse::<usize>().unwrap(),
        None => 1,
    };
    let input = File::open(filename)?;
    let reader = BufReader::new(input);
    let count;

    let readings: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();
    count = depth_changes(window_size, readings.as_slice());
    println!("depth changes: {}", count);
    Ok(())
}

fn depth_changes(window_size: usize, readings: &[usize]) -> usize {
    let mut count: usize = 0;
    for depth in 0..readings.len() - window_size {
        if readings[depth] < readings[depth + window_size] {
            count += 1;
        }
    }
    count
}
