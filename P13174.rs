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

const MOD: u64 = 1_000_000_007;

fn pow(a: u64, x: u64) -> u64 {
    let (mut r, mut a_t, mut x_t) = (1, a, x);
    while x_t != 0 {
        if x_t & 1 == 1 { r *= a_t; r %= MOD; }
        a_t *= a_t;
        a_t %= MOD;
        x_t >>= 1;
    }
    r
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let (n, k): (usize, usize) = (scan.token(), scan.token());
    let mut catalan: Vec<u32> = vec![0;(n>>1)+1];
    catalan[0] = 1;
    for i in 0..(n>>1) {
        catalan[i+1] = (((((((((i as u64)<<1)+1)<<1)%MOD)*pow((i+2) as u64, MOD-2))%MOD)*(catalan[i] as u64))%MOD) as u32;
    }
    let mut dp: Vec<u32> = vec![0;n+1];
    dp[0] = 1;
    for i in 0..n {
        if i & 1 == 1 {
            dp[i+1] = ((((k+1) as u64)*(dp[i] as u64))%MOD) as u32;
        } else {
            dp[i+1] = ((MOD + ((((k+1) as u64)*(dp[i] as u64))%MOD) - ((pow(k as u64, (i>>1) as u64)*(catalan[i>>1] as u64))%MOD))%MOD) as u32;
        }
    }
    writeln!(sout, "{}", dp[n]).ok();
}