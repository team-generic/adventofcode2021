use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Order {
    cmd: String,
    magnitude: usize,
}

#[derive(Debug)]
struct Position {
    depth: usize,
    hz: usize,
    aim: usize,
}
fn main() -> Result<(), std::io::Error> {
    let filename = std::env::args().nth(1).expect("no filename given");
    println!("{}", filename);
    let input = File::open(filename)?;
    let reader = BufReader::new(input);

    let readings: Vec<Order> = reader
        .lines()
        .map(|line| -> Order {
            let tmp = line.unwrap();
            let mut iter = tmp.split_whitespace();
            Order {
                cmd: match iter.next() {
                    Some(x) => x.to_string(),
                    None => "".to_string(),
                },
                magnitude: match iter.next() {
                    Some(x) => x.parse::<usize>().unwrap(),
                    None => 0,
                },
            }
        })
        .collect::<Vec<Order>>();
    println!("result: {}", orders_received(readings.as_slice()));
    Ok(())
}

fn orders_received(orders: &[Order]) -> usize {
    let mut pos = Position {
        depth: 0,
        hz: 0,
        aim: 0,
    };
    for order in orders {
        match order.cmd.as_str() {
            "forward" => {
                pos.hz += order.magnitude;
                pos.depth += pos.aim * order.magnitude;
            }
            "down" => {
                pos.aim += order.magnitude;
            }
            "up" => {
                pos.aim -= order.magnitude;
            }
            _ => println!("no orders"),
        }
    }
    println!("depth: {}, hz: {}", pos.depth, pos.hz);
    pos.depth * pos.hz
}
