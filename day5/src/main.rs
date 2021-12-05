use std::collections::HashMap;
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
    let mut map = HashMap::new();

    println!("{:?}", segments);
    for segment in segments {
        for point in segment {
            println!("point: {}, {}", point.x, point.y);
            let counter = map.entry(point).or_insert(0);
            *counter += 1;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

impl LineSegment {
    fn iter(&self) -> SegmentIter {
        SegmentIter {
            start: self.start,
            end: self.end,
            next: Some(self.start),
        }
    }
}
impl IntoIterator for LineSegment {
    type Item = Point;
    type IntoIter = SegmentIter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
struct SegmentIter {
    start: Point,
    end: Point,
    next: Option<Point>,
}
impl Iterator for SegmentIter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next = match self.next {
            None => return None,
            Some(n) => n,
        };
        if (self.start.x == self.end.x) ^ (self.start.y == self.end.y) == false {
            return None;
        }
        let ret = Some(next);
        if next == self.end {
            self.next = None;
        } else if self.start.y == self.end.y {
            if self.start.x > self.end.x {
                next.x -= 1;
            } else {
                next.x += 1;
            }
            self.next = Some(next);
        } else {
            if self.start.y > self.end.y {
                next.y -= 1;
            } else {
                next.y += 1;
            }
            self.next = Some(next);
        }
        ret
    }
}
