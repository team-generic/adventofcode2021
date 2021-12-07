use std::num::ParseIntError;
use std::str::FromStr;
fn main() {
    println!("Hello, world!");
}

struct Timer {
    countdown: isize,
}

impl FromStr for Timer {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let countdown_fromstr = s.parse::<isize>()?;
        Ok(Timer {
            countdown: countdown_fromstr,
        })
    }
}

impl Timer {
    fn iter(&self) -> TimerIter {
        TimerIter {
            start: self.countdown,
            next: Some(self.countdown),
            end: 0,
        }
    }

    fn spawn(&self) -> Timer {
        Timer { countdown: 8 }
    }

    fn restart(&mut self) {
        self.countdown = 6;
    }

    fn is_ready(&self) -> bool {
        self.countdown == 0
    }
}

struct TimerIter {
    start: isize,

    next: Option<isize>,
    end: isize,
}

impl IntoIterator for Timer {
    type Item = isize;
    type IntoIter = TimerIter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Iterator for TimerIter {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next;
        let mut next = match self.next {
            None => return None,
            Some(t) => t,
        };
        if next == self.end {
            self.next = None;
        } else {
            next -= 1;
            self.next = Some(next);
        }
        ret
    }
}
