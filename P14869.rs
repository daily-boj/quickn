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

const INF: i32 = 1_000_000_000;

use std::cmp::min;
use std::collections::VecDeque;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let (n, m, s, e, t): (usize, usize, usize, usize, usize) = (scan.token(), scan.token(), scan.token(), scan.token(), scan.token());
    let mut c: Vec<Vec<usize>> = vec![vec![0;m+s+1];n];
    for i in 0..n {
        let mut psum = 0;
        for j in 0..m {
            let a_ij: usize = scan.token();
            psum += a_ij;
            c[i][j+1] = psum;
        }
        for j in m..(m+s) {
            c[i][j+1] = c[i][m];
        }
    }
    let mut forbidden: Vec<usize> = Vec::with_capacity(n);
    for _i in 0..n {
        let mut f_i: usize = scan.token();
        f_i -= 1;
        forbidden.push(f_i);
    }
    let mut dp: Vec<Vec<i32>> = vec![vec![INF;m+s+1];n];
    let mut deque: Vec<VecDeque<(i32, usize)>> = vec![VecDeque::new();n];
    for i in 0..(m+s) {
        let i0 = (i as i32)-(s as i32);
        if i0 >= 0{
            let mut tmp: Vec<(i32, usize)> = vec![(0, 0);n];
            for j in 0..n {
                tmp[j] = (dp[j][i0 as usize], j);
            }
            tmp.sort_by(|(a, _b), (c, _d)| a.cmp(&c));
            for j in 0..n {
                let mut k = 0;
                while k < 3 {
                    let (mut ev, nj): (i32, usize) = (tmp[k].0, tmp[k].1);
                    if ev == INF {
                        break;
                    }
                    if nj != j && forbidden[j] != nj {
                        ev -= c[j][(i0+1) as usize] as i32;
                        while let Some(&(back, _i)) = deque[j].back() {
                            if back > ev {
                                deque[j].pop_back();
                            } else {
                                break;
                            }
                        }
                        deque[j].push_back((ev, i0 as usize));
                        break;
                    }
                    k += 1;
                }
            }
        }
        for j in 0..n {
            if i >= s-1 && i < e {
                dp[j][i] = c[j][i+1] as i32;
            }
            while let Some(&(_front, ni)) = deque[j].front() {
                if (ni as i32) < (i as i32) - (e as i32) {
                    deque[j].pop_front();
                } else {
                    break;
                }
            }
            if !deque[j].is_empty() {
                dp[j][i] = min(deque[j].front().unwrap().0 + (c[j][i+1] as i32) + (t as i32), dp[j][i]);
            }
        }
    }
    let mut res = INF;
    for i in 0..n {
        for j in (m-1)..(m+s) {
            res = min(res, dp[i][j]);
        }
    }
    //dbg!(dp.clone());
    writeln!(sout, "{}", res).ok();
}