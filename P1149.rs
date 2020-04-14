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

const INF: u32 = 1_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let n: usize = scan.token();
    let mut dp: Vec<Vec<u32>> = vec![vec![INF;3];n];
    let (r_0, g_0, b_0): (u32, u32, u32) = (scan.token(), scan.token(), scan.token());
    dp[0][0] = r_0;
    dp[0][1] = g_0;
    dp[0][2] = b_0;
    for i in 1..n {
        let (r, g, b): (u32, u32, u32) = (scan.token(), scan.token(), scan.token());
        // R
        for j in 0..3 {
            if j != 0 {
                dp[i][0] = min(dp[i][0], dp[i-1][j] + r);
            }
        }
        // G
        for j in 0..3 {
            if j != 1 {
                dp[i][1] = min(dp[i][1], dp[i-1][j] + g);
            }
        }
        // R
        for j in 0..3 {
            if j != 2 {
                dp[i][2] = min(dp[i][2], dp[i-1][j] + b);
            }
        }
    }
    writeln!(sout, "{}", min(dp[n-1][0], min(dp[n-1][1], dp[n-1][2]))).ok();
}