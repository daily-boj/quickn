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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Kind {
    Two,   // (
    Three, // [
    Value(u32),
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let s: String = scan.token();
    let mut stack: Vec<Kind> = Vec::new();
    let mut is_failed = false;
    for c in s.chars() {
        match c {
            '(' => stack.push(Kind::Two),
            '[' => stack.push(Kind::Three),
            ')' => {
                if stack.len() == 0 {
                    is_failed = true;
                    break;
                }
                match stack.pop().unwrap() {
                    Kind::Value(val) => {
                        if let Some(&res) = stack.last() {
                            if res != Kind::Two {
                                is_failed = true;
                                break;
                            } else {
                                stack.pop().unwrap();
                            }
                        } else {
                            is_failed = true;
                            break;
                        }
                        stack.push(Kind::Value(val << 1));
                    }
                    Kind::Two => {
                        stack.push(Kind::Value(2));
                    }
                    _ => {
                        is_failed = true;
                        break;
                    }
                }
            }
            _ => {
                if stack.len() == 0 {
                    is_failed = true;
                    break;
                }
                match stack.pop().unwrap() {
                    Kind::Value(val) => {
                        if let Some(&res) = stack.last() {
                            if res != Kind::Three {
                                is_failed = true;
                                break;
                            } else {
                                stack.pop().unwrap();
                            }
                        } else {
                            is_failed = true;
                            break;
                        }
                        stack.push(Kind::Value(val * 3));
                    }
                    Kind::Three => {
                        stack.push(Kind::Value(3));
                    }
                    _ => {
                        is_failed = true;
                        break;
                    }
                }
            }
        }
        let mut val = 0;
        while let Some(&kind) = stack.last() {
            match kind {
                Kind::Value(res) => {
                    val += res;
                    stack.pop().unwrap();
                }
                _ => {
                    break;
                }
            }
        }
        if val != 0 {
            stack.push(Kind::Value(val));
        }
    }
    if is_failed || stack.len() != 1 {
        writeln!(sout, "0").ok();
    } else {
        match stack.pop().unwrap() {
            Kind::Value(res) => {
                writeln!(sout, "{}", res).ok();
            }
            _ => {
                writeln!(sout, "0").ok();
            }
        };
    }
}
