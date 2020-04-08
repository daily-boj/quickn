/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

use std::str;
use std::io::{self, BufWriter, Write};

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

const N: usize = 10000;
const MAX_STEP: usize = 500;

fn solve(a: f64, c_i: f64, y_p1: f64) -> (f64, f64) {
    let func = |x: f64, y: f64, y_dot: f64| -> f64 {
        ((2.0*(1.0+y_dot.powi(2))*(y_dot*(x+(y-c_i)*y_dot) - (y-c_i)*(1.0+y_dot.powi(2))))/(x.powi(2)+(y-c_i).powi(2)).powi(2))/(1.0 + 1.0/(x.powi(2)+(y-c_i).powi(2)))
    };
    let func2 = |x: f64, y: f64, y_dot: f64| -> f64 {
        (1.0 + y_dot.powi(2)).sqrt()*(1.0 + 1.0/(x.powi(2) + (y-c_i).powi(2)))
    };
    let (mut x0, mut y0, mut y_p0): (f64, f64, f64) = (-10.0, a, y_p1 as f64);
    let h = 20.0/(N as f64);
    let mut sum: f64 = func2(x0, y0, y_p0);
    {
        for k in 1..=(N+1) {
            let k1 = [func(x0, y0, y_p0), y_p0];
            let k2 = [func(x0 + h/2.0, y0 + ((h*k1[1])/2.0), y_p0 + (h*k1[0])/2.0), y_p0 + ((h*k1[0])/2.0)];
            let k3 = [func(x0 + h/2.0, y0 + ((h*k2[1])/2.0), y_p0 + (h*k2[0])/2.0), y_p0 + ((h*k2[0])/2.0)];
            let k4 = [func(x0 + h, y0 + (h*k3[1]), y_p0 + (h*k3[0])), y_p0 + (h*k3[0])];
            let (x, y, y_p): (f64, f64, f64) = (-10.0 + (k as f64)*h, y0 + ((h*(k1[1] + 2.0*k2[1]+2.0*k3[1] + k4[1]))/6.0), y_p0 + ((h*(k1[0] + 2.0*k2[0]+2.0*k3[0] + k4[0]))/6.0));
            x0 = x;
            y0 = y;
            y_p0 = y_p;
            sum += func2(x0, y0, y_p0);
        }
    }
    (h*sum, y0)
}

fn main() {
    let (stdio, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdio.lock()), BufWriter::new(stdout.lock()));
    let t: usize = scan.token();
    for i in 0..t {
        let (_n, a, b): (usize, f64, f64) = (scan.token(), scan.token(), scan.token());
        let c_i: f64 = scan.token();
        let (mut l, mut r): (f64, f64) = (-13.0, -a);
        let mut cnt = 0;
        while (r-l).abs() > std::f64::EPSILON && cnt < MAX_STEP {
            let mid = (l+r)/2.0;
            let func = solve(a, c_i, mid);
            if func.1 < b {
                l = mid;
            } else if func.1 > b {
                r = mid;
            }
            cnt += 1;
        }
        let v1 = solve(a, c_i, l).0;
        l = -a;
        r = 13.0;
        cnt = 0;
        while (r-l).abs() > std::f64::EPSILON && cnt < MAX_STEP {
            let mid = (l+r)/2.0;
            let func = solve(a, c_i, mid);
            if func.1 <= b {
                l = mid;
            } else if func.1 > b {
                r = mid;
            }
            cnt += 1;
        }
        let v2 = solve(a, c_i, l).0;
        if v1 < v2 {
            writeln!(sout, "Case #{}: {:.3}", i+1, v1).ok();
        } else {
            writeln!(sout, "Case #{}: {:.3}", i+1, v2).ok();
        }
    }
}