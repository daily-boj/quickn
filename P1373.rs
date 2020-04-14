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
    // Big-endian
    let s: String = scan.token();
    let mut s_chars: Vec<char> = s.chars().collect();
    // Little-endian
    s_chars.reverse();
    let mut res: String = String::new();
    let mut tmp = 0;
    for i in 0..s_chars.len() {
        let t = (s_chars[i] as u8) - ('0' as u8);
        tmp += t*((1 << (i % 3)) as u8);
        if i % 3 == 2 {
            res.push_str(tmp.to_string().as_str());
            tmp = 0;
        }
    }
    if tmp != 0 || res.len() == 0 {
        res.push_str(tmp.to_string().as_str());
    }
    writeln!(sout, "{}", res.chars().rev().collect::<String>()).ok();
}