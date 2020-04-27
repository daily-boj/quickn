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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut arr1: Vec<i64> = vec![0; n];
    let mut arr2: Vec<i64> = vec![0; n];
    for i in 0..n {
        arr1[i] = scan.token();
    }
    for i in 0..n {
        arr2[i] = scan.token();
    }
    // 1
    let mut brute1: Vec<(i64, u64)> = vec![(0, 0)];
    for i in 0..(n >> 1) {
        let init_len = brute1.len();
        for j in 0..init_len {
            brute1.push((brute1[j].0 - arr2[i], brute1[j].1 | (1 << (n - i - 1))));
            brute1[j].0 += arr1[i];
        }
    }
    // 2
    let mut brute2: Vec<(i64, u64)> = vec![(0, 0)];
    for i in (n >> 1)..n {
        let init_len = brute2.len();
        for j in 0..init_len {
            brute2.push((brute2[j].0 - arr2[i], brute2[j].1 | (1 << (n - i - 1))));
            brute2[j].0 += arr1[i];
        }
    }
    brute1.sort_by(|(ans1, _mask1), (ans2, _mask2)| ans1.cmp(&ans2));
    //dbg!(brute1.clone(), brute2.clone());
    let mut res: Vec<(i64, u64)> = Vec::new();
    for (ans, mask) in brute2 {
        let (mut s, mut e) = (0, brute1.len());
        while s + 3 <= e {
            let l = ((s << 1) + e) / 3;
            let r = (s + (e << 1)) / 3;
            let cost_l = ans + brute1[l].0;
            let cost_r = ans + brute1[r].0;
            if cost_l < cost_r {
                e = r;
            } else {
                s = l;
            }
        }
        let mut idx = s;
        for i in s..=e {
            if (ans + brute1[idx].0).abs() > (ans + brute1[i].0).abs()
                || ((ans + brute1[idx].0).abs() == (ans + brute1[i].0).abs()
                    && brute1[idx].1 > brute1[i].1)
            {
                idx = i;
            }
        }
        res.push((ans + brute1[idx].0, mask | brute1[idx].1));
    }
    let mut idx = 0;
    for i in 1..res.len() {
        if res[idx].0.abs() > res[i].0.abs()
            || (res[idx].0.abs() == res[i].0.abs() && res[idx].1 > res[i].1)
        {
            idx = i;
        }
    }
    for i in 0..n {
        if res[idx].1 & (1 << (n - i - 1)) == 0 {
            write!(sout, "1 ").ok();
        } else {
            write!(sout, "2 ").ok();
        }
    }
    writeln!(sout, "").ok();
}
