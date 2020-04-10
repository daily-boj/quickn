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

const MAX: usize = 2001;

#[derive(Clone)]
struct SegmentTree {
    len: usize,
    data: Box<[i64;MAX]>,
    data2: Box<[i64;MAX]>,
}

impl SegmentTree {
    fn new(len: usize) -> Self {
        let data: Box<[i64;MAX]> = Box::new([0;MAX]);
        let data2: Box<[i64;MAX]> = Box::new([0;MAX]);
        Self {
            len,
            data,
            data2,
        }
    }

    fn update(&mut self, pos: usize, diff: i64, diff2: i64) {
        let mut pos_t = pos;
        while pos_t <= self.len {
            self.data[pos_t] += diff;
            self.data2[pos_t] += diff2;
            pos_t += ((pos_t as i64) & -(pos_t as i64)) as usize;
        }
    }

    fn update_range(&mut self, l: usize, r: usize, diff: i64) {
        self.update(l, diff, -diff*((l as i64)-1));
        self.update(r+1, -diff, diff*((r as i64)));
    }

    fn query(&self, i: usize) -> i64 {
        if i == 0 {
            return 0;
        }
        let (mut res, mut res2) = (0, 0);
        let mut i_t = i as i64;
        while i_t > 0 {
            res += self.data[i_t as usize];
            res2 += self.data2[i_t as usize];
            i_t -= i_t & -i_t;
        }
        ((i as i64)*res) + res2
    }

    fn range_query(&self, l: usize, r: usize) -> i64 {
        self.query(r) - self.query(l-1)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let n: usize = scan.token();
    let mut jogae: Vec<Vec<i64>> = vec![vec![0;n];n];
    let mut dp: Vec<Vec<i64>> = vec![vec![0;n];n];
    let mut seg: Vec<SegmentTree> = Vec::with_capacity(n);
    for i in 0..n {
        for j in 0..n {
            jogae[i][j] = scan.token();
            let d1 = if i > 0 { dp[j][i-1] } else { 0 };
            let d2 = if j > 0 { dp[j-1][i] } else { 0 };
            dp[j][i] = jogae[i][j] + max(d1, d2);
        }
    }
    for j in 0..n {
        let mut seg_i = SegmentTree::new(n);
        for i in 1..=n {
            seg_i.update_range(i, i, dp[j][i-1]);
        }
        seg.push(seg_i);
    }
    let mut res = 0;
    for j in 0..n {
        res += seg[j].query(n);
    }
    writeln!(sout, "{}", res).ok();
    for _i in 0..n {
        let (act, a, b): (String, usize, usize) = (scan.token(), scan.token(), scan.token());
        let d: bool = match act.as_str() {
            "U" => {
                false
            },
            _ => {
                true
            },
        };
        if d {
            jogae[a-1][b-1] -= 1;
        } else {
            jogae[a-1][b-1] += 1;
        }
        let (mut s, mut e) = (a-1, n-1);
        for col in (b-1)..n {
            let mut row = s;
            let mut is_mut = false;
            while row <= e {
                let d1 = jogae[row][col] + if row > 0 { seg[col].range_query(row, row) } else { 0 };
                let d2 = jogae[row][col] + if col > 0 { seg[col-1].range_query(row+1, row+1) } else { 0 };
                if max(d1, d2) != seg[col].range_query(row+1, row+1) {
                    is_mut = true;
                    break;
                }
                row += 1;
            }
            if !is_mut {
                break;
            }
            s = row;
            if col != b-1 {
                row = e;
            }
            if col != b-1 && s < e {
                seg[col].update_range(s+1, e, if d { -1 } else { 1 });
            }
            while row < n {
                let d1 = jogae[row][col] + if row > 0 { seg[col].range_query(row, row) } else { 0 };
                let d2 = jogae[row][col] + if col > 0 { seg[col-1].range_query(row+1, row+1) } else { 0 };
                if max(d1, d2) == seg[col].range_query(row+1, row+1) {
                    break;
                } else {
                    seg[col].update_range(row+1, row+1, if d { -1 } else { 1 });
                }
                row += 1;
            }
            row -= 1;
            e = row;
        }
        res = 0;
        for j in 0..n {
            res += seg[j].query(n);
        }
        writeln!(sout, "{}", res).ok();
    }
}