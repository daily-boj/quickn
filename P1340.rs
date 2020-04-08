/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

use std::str;
use std::io::{self, BufWriter, Write};

/* https://github.com/EbTech/rust-algorithms */

/// Same API as Scanner but nearly twice as fast, using horribly unsafe dark arts
/// **REQUIRES** Rust 1.34 or higher
pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    /// This function should be marked unsafe, but noone has time for that in a
    /// programming contest. Use at your own risk!
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

const DAYS: [u32;12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const DAYS2: [u32;12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let (month, mut day, year, time): (String, String, u32, String) = (scan.token(), scan.token(), scan.token(), scan.token());
    day = day.replace(",","");
    let month2num = |month: &str| -> usize {
        match month {
            "January" => { 0 },
            "February" => { 1 },
            "March" => { 2 },
            "April" => { 3 },
            "May" => { 4 },
            "June" => { 5 },
            "July" => { 6 },
            "August" => { 7 },
            "September" => { 8 },
            "October" => { 9 },
            "November" => { 10 },
            _ => { 11 },
        }
    };
    let split_time: Vec<&str> = time.split(":").collect();
    let (hour, minute): (u32, u32) = (split_time[0].parse().unwrap(), split_time[1].parse().unwrap());
    if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        let mut psum = 0;
        for i in 0..month2num(&month) {
            psum += DAYS2[i];
        }
        psum += day.parse::<u32>().unwrap()-1;
        writeln!(sout, "{}", (((psum as f64) + (((hour*60 + minute) as f64)/1440.0))/366.0)*100.0).ok();
    } else {
        let mut psum = 0;
        for i in 0..month2num(&month) {
            psum += DAYS[i];
        }
        psum += day.parse::<u32>().unwrap()-1;
        writeln!(sout, "{}", (((psum as f64) + (((hour*60 + minute) as f64)/1440.0))/365.0)*100.0).ok();
    }
}
