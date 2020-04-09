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

use std::cmp::max;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let (n, m): (usize, usize) = (scan.token(), scan.token());
    let mut daily = vec![0;n];
    let mut max_v = 0;
    for i in 0..n {
        daily[i] = scan.token();
        max_v = max(daily[i], max_v);
    }
    let (mut l, mut r) = (max_v, 1_000_000_000);
    let f = |mid: usize| -> usize {
        if max_v > mid {
            return 0;
        }
        let mut func = 0;
        let mut left = 0;
        for i in 0..n {
            if left < daily[i] {
                left = mid;
                func += 1;
            }
            left -= daily[i];
        }
        return func;
    };
    while l <= r {
        let mid = (l+r)>>1;
        let func = f(mid);
        if func > m {
            l = mid+1;
        } else {
            r = mid-1;
        }
    }
    while l > 1 && f(l-1) == f(l) {
        l -= 1;
    }
    writeln!(sout, "{}", l).ok();
}