use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    let filename = std::env::args().nth(1).expect("no filename given");
    println!("{}", filename);
    let input = File::open(filename).expect("could not open file");
    let reader = BufReader::new(input);
    let lines_iter = reader.lines().map(|l| l.unwrap());
    let segments: Vec<LineSegment> = lines_iter
        .map(|line| -> LineSegment { line.parse::<LineSegment>().unwrap() })
        .collect();
    println!("{:?}", segments);
}

#[derive(Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        let x_fromstr = coords[0].parse::<isize>()?;
        let y_fromstr = coords[1].parse::<isize>()?;
        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

impl FromStr for LineSegment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split_whitespace().collect();
        let start = coords[0].parse::<Point>()?;
        let end = coords[2].parse::<Point>()?;
        assert_eq!(coords[1], "->");
        Ok(LineSegment { start, end })
    }
}
