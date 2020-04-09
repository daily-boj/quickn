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
    let (_n, m): (usize, usize) = (scan.token(), scan.token());
    let k: usize = scan.token();
    let mut init_set: u64 = 0;
    for _i in 0..k {
        let j: u8 = scan.token();
        init_set |= 1 << (j-1);
    }
    let mut party: Vec<u64> = vec![0;m];
    let mut dp: Vec<Vec<(u64, u64, usize)>> = vec![vec![];m+1];
    for i in 0..m {
        let num: usize = scan.token();
        for _j in 0..num {
            let j: u8 = scan.token();
            party[i] |= 1 << (j-1);
        }
    }
    let mut res = 0;
    dp[0].push((init_set, 0, 0));
    for i in 1..=m {
        for (t, f, depth) in dp[i-1].clone() {
            if party[i-1] & t == 0 {
                dp[i].push((t, f | party[i-1], depth+1));
            }
            if party[i-1] & f == 0 {
                dp[i].push((t | party[i-1], f, depth));
            }
        }
    }
    for (_t, _f, depth) in dp[m].clone() {
        res = max(res, depth);
    }
    writeln!(sout, "{}", res).ok();
}