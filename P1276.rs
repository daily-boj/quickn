/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

use std::io::{self, BufWriter, Write};
use std::str;

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

use std::cmp::{Ord, Ordering, PartialOrd};
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Segment {
    x1: u32,
    x2: u32,
    y: u32,
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Segment) -> Option<Ordering> {
        Some(if self.y > other.y { Ordering::Less } else { Ordering::Greater }) 
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Segment) -> Ordering {
        if self.y > other.y { Ordering::Less } else { Ordering::Greater } 
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut segments: Vec<Segment> = Vec::with_capacity(n);
    for _i in 0..n {
        let (y, x1, x2): (u32, u32, u32) = (scan.token(), scan.token(), scan.token());
        segments.push(Segment { x1, x2, y });
    }
    let mut ans = 0;
    segments.sort();
    //dbg!(segments.clone());
    for i in 0..n {
        ans += segments[i].y << 1;
        for j in (i+1)..n {
            if segments[j].x1 <= segments[i].x1 
                && segments[i].x1 < segments[j].x2 
            {
                ans -= segments[j].y;
                break;
            }
        }
        for j in (i+1)..n {
            if (segments[j].x1 as i32) <= (segments[i].x2 as i32) - 1
                && (segments[i].x2 as i32) - 1 < segments[j].x2 as i32
            {
                ans -= segments[j].y;
                break;
            }
        }
    }
    writeln!(sout, "{}", ans).ok();
}
