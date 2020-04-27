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
    let a: usize = scan.token();
    let mut t: usize = scan.token();
    let kind: u8 = scan.token();
    t -= 1;
    if kind == 0 {
        let n = ((-7.0 + (49.0 + (t<<3) as f64).sqrt())/2.0).floor() as usize;
        let b = t - (n*(n+1)>>1) - 3*n;
        let c = (((6*n) % a) + ((n*(n+1)) % a)) % a;
        if b == 0 {
            writeln!(sout, "{}", c).ok();
        } else if b == 1 {
            writeln!(sout, "{}", (c+2)%a).ok();
        } else {
            writeln!(sout, "{}", (c+2+b)%a).ok();
        }
    } else {
        let n = ((-7.0 + (49.0 + (t<<3) as f64).sqrt())/2.0).floor() as usize;
        let b = t - (n*(n+1)>>1) - 3*n;
        let c = (((6*n) % a) + ((n*(n+1)) % a)) % a;
        if b == 0 {
            writeln!(sout, "{}", (c+1)%a).ok();
        } else if b == 1 {
            writeln!(sout, "{}", (c+3)%a).ok();
        } else {
            writeln!(sout, "{}", (c+4+n+b)%a).ok();
        }
    }
}