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
    let mut dp: Vec<Vec<Vec<u64>>> = vec![vec![vec![0;3];m];n];
    let mut cost: Vec<Vec<u64>> = vec![vec![0;m];n];
    for i in 0..n {
        for j in 0..m {
            cost[i][j] = scan.token();
        }
    }
    let mut res = 0;
    for j in 0..m {
        for i in 0..n {
            if j == 0 {
                dp[i][j][0] = cost[i][j];
                dp[i][j][1] = cost[i][j];
                dp[i][j][2] = cost[i][j];
            } else {
                dp[i][j][1] = dp[i][j-1][2] + cost[i][j];
                dp[i][j][2] = dp[i][j-1][1] + cost[i][j];
                if j >= 2 {
                    dp[i][j][0] = max(dp[i][j][0], dp[i][j-2][0] + cost[i][j] + cost[i][j-1]);
                }
                if i > 0 {
                    dp[i][j][0] = max(dp[i][j][0], dp[i-1][j-1][0] + cost[i][j]);
                    dp[i][j][2] = max(dp[i][j][2], dp[i-1][j-1][2] + cost[i][j]);
                }
                if i < n-1 {
                    dp[i][j][0] = max(dp[i][j][0], dp[i+1][j-1][0] + cost[i][j]);
                    dp[i][j][1] = max(dp[i][j][1], dp[i+1][j-1][1] + cost[i][j]);
                }
            }
            if j == m-1 {
                res = max(dp[i][j][0], res);
                res = max(dp[i][j][1], res);
                res = max(dp[i][j][2], res);
            }
        }
    }
    writeln!(sout, "{}", res).ok();
}