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

use std::cmp::{max, min};
use std::mem::swap;

fn fast_gcd(n: u32, m: u32) -> u32 {
    if n == 0 || m == 0 { return max(n, m); }
    let (mut n_t, mut m_t) = (max(n, m), min(n, m));
    while m_t != 0 {
        n_t %= m_t;
        swap(&mut n_t, &mut m_t);
    }
    return n_t;
}

macro_rules! gcd {
    ($ ($ x: expr), *) => {{
        let mut res = 0;
        $(
            res = fast_gcd(res, $x);
        )*
        res
    }};
}

macro_rules! min {
    ($ ($ x: expr), *) => {{
        let mut res = u32::MAX;
        $(
            res = min(res, $x);
        )*
        res
    }};
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (mut a, mut b, mut c): (u32, u32, u32) = (scan.token(), scan.token(), scan.token());
    let (mut i, mut j, mut k): (u32, u32, u32) = (scan.token(), scan.token(), scan.token());
    let d = gcd![i, j, k];
    dbg!(d);
    i /= d;
    j /= d;
    k /= d;
    while a >= i && b >= j && c >= k {
        a -= i;
        b -= j;
        c -= k;
    }
    let min_val = min![a, b, c];
    writeln!(sout, "{} {} {}", (a as f64) - (min_val as f64)/(a as f64), (b as f64) - (min_val as f64)/(b as f64), (c as f64) - (min_val as f64)/(c as f64)).ok();
}
 