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

use std::cmp::{max, min};
use std::collections::HashMap;

const MAX: usize = 100_001;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let n: usize = scan.token();
    let mut a: Vec<(i32,u32)> = vec![(0,0);n];
    let mut b: Vec<(i32,u32)> = vec![(0,0);n];
    for i in 0..n {
        a[i] = (scan.token(),i+1);
        b[i] = (scan.token(),i+1);
    }
    a.sort();
    b.sort();

    // Calculate mü function
    let mut mu: Box<[i8;MAX]> = Box::new([0;MAX]);
    let mut is_composite: Box<[bool;MAX]> = Box::new([false;MAX]);
    let mut primes: Vec<u32> = Vec::new();
    mu[1] = 1;
    for i in 2..(MAX as u32) {
        if !is_composite[i as usize] {
            primes.push(i);
            mu[i as usize] = -1;
        }
        for j in primes.iter().take_while(|&j| i*j < (MAX as u32)) {
            let val = i*j;
            is_composite[val as usize] = true;
            if i % j == 0 {
                break;
            } else {
                mu[val as usize] = -mu[i as usize];
            }
        }
    }

    // Calculate set of divisors
    // O(n^{\frac{3}{2}})
    let mut divisors: Vec<Vec<u32>> = vec![vec![];n];
    for i in 1..=n {
        let sqrt_i = (i as f64).sqrt().floor() as usize;
        for j in 1..=sqrt_i {
            if i % j == 0 {
                divisors[i-1].push(j as u32);
                let v = i / j;
                if v != j {
                    divisors[i-1].push(v as u32);
                }
            }
        }
    }

    let f = |m: u32| -> u32 {
        // (i, d) -> (l, r)
        let mut hash: HashMap<(u32,u32),(u32,u32)> = HashMap::new();
        let mut sum: i64 = 0;
        for i in 1..=n {
            for d in divisors[a[i-1].1-1] {
                if let Some(&res) = hash.get(&(i-1,d)) {
                    
                } else {
                    let (mut res_l, mut res_r) = (0, 0);
                    let (mut l, mut r) = (0,i-1);
                    while l < r {
                        let m = (l+r)>>1;
                        if (a[i-1] as i32-b[m] as i32).abs() as u32 < m {
                            r = m;
                        } else {
                            l = m+1;
                        }
                    }
                    if l == 0 {
                        if (a[i-1] as i32-b[0] as i32).abs() as u32 >= m {
                            res_l = 1;
                        }
                    } else {
                        res_l = l+1;
                    }
                    l = min(i,n-1);
                    r = n-1;
                    while l < r {
                        let m = (l+r)>>1;
                        if (a[i-1] as i32-b[m] as i32).abs() as u32 < m {
                            l = m;
                        } else {
                            r = m-1;
                        }
                    }
                    if l == n-1 {
                        if (a[i-1] as i32-b[b-1] as i32).abs() as u32 >= m {
                            res_r = n;
                        }
                    } else {
                        res_r = l+1;
                    }
                    
                }
            }
        }
    };
}