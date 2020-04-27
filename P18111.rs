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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let (n, m, b): (usize, usize, u32) = (scan.token(), scan.token(), scan.token());
    let mut board: Vec<Vec<u32>> = vec![vec![0;m];n];
    for i in 0..n {
        for j in 0..m {
            board[i][j] = scan.token();
        }
    }
    let (mut min_t, mut max_h) = (1_000_000_000, 0);
    for h in 0..=256 {
        let (mut under, mut over): (u32, u32) = (0, 0);
        for i in 0..n {
            for j in 0..m {
                if h < board[i][j] {
                    over += board[i][j] - h;
                } else if h > board[i][j] {
                    under += h - board[i][j];
                }
            }
        }
        if over + b >= under {
            let t = (over<<1) + under;
            if min_t > t {
                min_t = t;
                max_h = h;
            } else if min_t == t && max_h < h {
                max_h = h;
            }
        }
    }
    writeln!(sout, "{} {}", min_t, max_h).ok();
}