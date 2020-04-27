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

extern crate core;

use core::arch::x86_64::_rdrand64_step;
use std::cmp::{max, min};
use std::mem::swap;

const PRIMES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

fn fast_gcd(n: u64, m: u64) -> u64 {
    if n == 0 || m == 0 {
        return max(n, m);
    }
    let mut n_t = max(n, m);
    let mut m_t = min(n, m);
    while m_t != 0 {
        n_t %= m_t;
        swap(&mut n_t, &mut m_t);
    }
    n_t
}

fn g(x: u64, n: u64) -> u64 {
    ((n as u128 + (x as u128 * x as u128) - 1) % (n as u128)) as u64
}

fn pow_mod(a: u128, x: u128, n: u128) -> u128 {
    let mut a_t = a;
    let mut x_t = x;
    let mut r = 1;
    while x_t != 0 {
        if (x_t & 1) == 1 {
            r = (r * a_t) % n;
        }
        a_t *= a_t;
        a_t %= n;
        x_t >>= 1;
    }
    r
}

fn witness(a: u64, n: u64) -> bool {
    let mut u = n - 1;
    let mut t = 0;
    while u % 2 == 0 {
        u >>= 1;
        t += 1;
    }
    let mut x = pow_mod(a as u128, u as u128, n as u128) as u64;
    if x == 1 || x == n - 1 {
        return false;
    }
    for _i in 0..(t - 1) {
        x = ((x as u128 * x as u128) % (n as u128)) as u64;
        if x == n - 1 {
            return false;
        }
    }
    return true;
}

fn miller_rabin(n: u64) -> bool {
    for p in &PRIMES {
        if *p >= n {
            break;
        }
        if witness(*p, n) {
            return true;
        }
    }
    return false;
}

fn pollard_rho(n: u64) -> u64 {
    let mut i = 1;
    let mut x = 0;
    unsafe {
        _rdrand64_step(&mut x);
    }
    x = x % n;
    let mut y = x;
    let mut k = 2;
    let mut d: u64;
    loop {
        i += 1;
        x = g(x, n);
        d = fast_gcd((y as i64 - x as i64).abs() as u64, n);
        if d != 1 {
            break;
        }
        if i == k {
            y = x;
            k <<= 1;
        }
    }
    d
}

fn factorization(m: u64) -> Vec<u64> {
    let mut n = m;
    let mut factors: Vec<u64> = Vec::new();
    while n & 1 != 1 {
        factors.push(2);
        n >>= 1;
    }
    while n != 1 {
        if !miller_rabin(n) {
            factors.push(n);
            break;
        }
        let mut x = pollard_rho(n);
        while miller_rabin(x) {
            x = pollard_rho(x);
        }
        n /= x;
        factors.push(x);
    }
    factors
}

use std::mem::MaybeUninit;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let k: u32 = scan.token();
    let mut f = unsafe { MaybeUninit::zeroed().assume_init() };
    let f_rec = &mut f as *mut dyn FnMut(u32, u32) -> u32;
    f = |m: u32, depth: u32| -> u32 {
        if m == 1 || !miller_rabin(m as u64) {
            depth
        } else {
            let factors = factorization(m as u64);
            let (mut left, mut right) = (1, 1);
            for i in 0..(factors.len() >> 1) {
                left *= factors[i] as u32;
                right *= factors[factors.len() - i - 1] as u32;
            }
            if factors.len() & 1 == 1 {
                right *= factors[factors.len()>>1] as u32;
            }
            unsafe { max((*f_rec)(left, depth + 1), (*f_rec)(right, depth + 1)) }
        }
    };
    writeln!(sout, "{}", f(k, 0)).ok();
}
