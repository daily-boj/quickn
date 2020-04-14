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

use std::cmp::min;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let (mut cnt1, mut cnt2, mut cnt3): (usize, usize, usize) = (0, 0, 0);
    let n: usize = scan.token();
    for _i in 0..n {
        let input: String = scan.token();
        match input.as_str() {
            "1/4" => {
                cnt1 += 1;
            },
            "1/2" => {
                cnt2 += 1;
            },
            _ => {
                cnt3 += 1;
            },
        }
    }
    let mut res = 0;
    // 1
    let t1 = min(cnt1, cnt3);
    res += t1;
    cnt1 -= t1;
    cnt3 -= t1;
    // 2
    let t2 = min(cnt1>>1, cnt2);
    res += t2;
    cnt1 -= t2<<1;
    cnt2 -= t2;
    // 3
    let t3 = min(cnt1, cnt2);
    res += t3;
    cnt1 -= t3;
    cnt2 -= t3;
    // 4
    res += ((cnt1 as f64)/4.0).ceil() as usize;
    res += ((cnt2 as f64)/2.0).ceil() as usize;
    res += cnt3;
    writeln!(sout, "{}", res).ok();
}