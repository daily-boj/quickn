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

use std::cmp::max;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (k, n): (usize, usize) = (scan.token(), scan.token());
    let mut arr: Vec<usize> = vec![0; k];
    let mut max_val = 0;
    for i in 0..k {
        arr[i] = scan.token();
        max_val = max(max_val, arr[i]);
    }
    let f = |mid: usize| -> usize {
        let mut func = 0;
        for i in 0..k {
            func += arr[i] / mid;
        }
        func
    };
    let (mut l, mut r) = (1, max_val+1);
    while l+1 < r {
        let mid = (l + r) >> 1;
        let func = f(mid);
        //dbg!(func);
        if func >= n {
            l = mid;
        } else {
            r = mid;
        }
    }
    writeln!(sout, "{}", l).ok();
}
