/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

extern crate core;

use core::arch::x86_64::_rdrand64_step;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::mem::swap;
const PRIMES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

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

fn factorization(m: u64) -> Vec<(u64, u8)> {
    let mut n = m;
    let mut hash: HashMap<u64, u8> = HashMap::new();
    let mut factors: Vec<u64> = Vec::new();
    while n & 1 != 1 {
        if let Some(&res) = hash.get(&2) {
            hash.insert(2, res + 1);
        } else {
            hash.insert(2, 1);
            factors.push(2);
        }
        n >>= 1;
    }
    while n != 1 {
        if !miller_rabin(n) {
            if let Some(&res) = hash.get(&n) {
                hash.insert(n, res + 1);
            } else {
                hash.insert(n, 1);
                factors.push(n);
            }
            break;
        }
        let mut x = pollard_rho(n);
        while miller_rabin(x) {
            x = pollard_rho(x);
        }
        n /= x;
        if let Some(&res) = hash.get(&x) {
            hash.insert(x, res + 1);
        } else {
            hash.insert(x, 1);
            factors.push(x);
        }
    }
    factors.iter().map(|&factor| (factor, *hash.get(&factor).unwrap())).collect()
}

fn fast_pow(a: u64, x: u64, p: u64) -> u64 {
    let mut r = 1;
    let mut a_t: u128 = a as u128;
    let mut x_t = x;
    while x_t != 0 {
        if (x_t & 1) == 1 {
            r = (((r as u128) * a_t) % (p as u128)) as u64;
        }
        a_t = (a_t * a_t) % (p as u128);
        x_t >>= 1;
    }
    r
}

fn fast_pow_normal(a: u64, x: u64) -> u64 {
    let mut r = 1;
    let mut a_t: u128 = a as u128;
    let mut x_t = x;
    while x_t != 0 {
        if (x_t & 1) == 1 {
            r *= a_t as u64;
        }
        a_t *= a_t;
        x_t >>= 1;
    }
    r
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let m: u64 = scan.token();
    let k: u8 = scan.token();
    let mut res: u128 = 0;
    let factors: Vec<(u64, u64, u8)> = factorization(m)
        .iter()
        .map(|&(factor, exp)| (fast_pow_normal(factor, exp as u64), factor, exp))
        .collect();
    let mut failed = false;
    //dbg!(factors.clone());
    for &(eval, factor, exp) in &factors {
        let phi = (factor - 1) * fast_pow_normal(factor, (exp - 1) as u64);
        if factor == 2 {
            if exp-1 < k+1 {
                failed = true;
                break;
            }
            let mut product: u128 = (1 << (exp - k)) + 1;
            for &(other, other_factor, _) in &factors {
                if other_factor != factor {
                    product *= other as u128;
                    product *= fast_pow(other % eval, phi - 1, eval) as u128;
                }
            }
            res += product;
        } else {
            let phi_factors: Vec<(u64, u64, u8)> = factorization(phi)
                .iter()
                .map(|&(factor, exp)| (fast_pow_normal(factor, exp as u64), factor, exp))
                .collect();
            if (factor-1) % (1 << k) != 0 {
                failed = true;
                break;
            }
            let mut g: u64 = 2;
            // Caculate primitive root of p_{i}^{e_{i}}
            // It costs O(log_{7 + \epsilon}{p_{i}^{e_{i}}}) by generalized Riemann hypothesis
            while g < eval {
                if fast_gcd(g, eval) == 1 {
                    let mut is_primitive = true;
                    for &(_, p, _) in &phi_factors {
                        if fast_pow(g, phi / p, eval) == 1 {
                            is_primitive = false;
                            break;
                        }
                    }
                    if is_primitive {
                        // Success
                        break;
                    }
                }
                g += 1;
            }
            let mut product: u128 = fast_pow(g, phi / (1 << k), eval) as u128;
            for &(other, other_factor, _) in &factors {
                if other_factor != factor {
                    product *= other as u128;
                    product *= fast_pow(other % eval, phi - 1, eval) as u128;
                }
            }
            res += product;
        }
    }
    if failed {
        write!(sout, "-1").ok();
    } else {
        write!(sout, "{}", res).ok();
    }
}
