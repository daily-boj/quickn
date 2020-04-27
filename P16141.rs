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

const MOD: i64 = 1_000_000_007;

fn pow(a: i64, x: usize) -> i64 {
    let (mut r, mut a_t, mut x_t) = (1, a, x);
    while x_t != 0 {
        if x_t & 1 == 1 {
            r *= a_t;
            r %= MOD;
        }
        a_t *= a_t;
        a_t %= MOD;
        x_t >>= 1;
    }
    r
}

use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::cmp::max;

const BUCKET: usize = 1_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, v): (usize, usize) = (scan.token(), scan.token());
    let mut binomial: Vec<Vec<i32>> = vec![vec![0; v + 2]; v + 2];
    for i in 1..=(v + 1) {
        binomial[i][0] = 1;
        binomial[i][i] = 1;
        for j in 1..i {
            binomial[i][j] = (binomial[i - 1][j] + binomial[i - 1][j - 1]) % (MOD as i32);
        }
    }
    let mut bernoulli: Vec<i32> = vec![0; v + 2];
    bernoulli[0] = 1;
    for m in 1..=(v + 1) {
        let mut sum = 1;
        for k in 0..m {
            sum -= ((((binomial[m][k] as i64) * pow((m - k + 1) as i64, (MOD - 2) as usize))
                % MOD)
                * (bernoulli[k] as i64))
                % MOD;
            sum %= MOD;
        }
        bernoulli[m] = sum as i32;
    }
    let mut mu: Box<[i8; BUCKET + 1]> = Box::new([0; BUCKET + 1]);
    let mut s_mu: Box<[i32; BUCKET + 1]> = Box::new([0; BUCKET + 1]);
    {
        let mut is_composite: Box<[bool; BUCKET + 1]> = Box::new([false; BUCKET + 1]);
        let mut primes: Vec<u32> = Vec::new();
        mu[1] = 1;
        s_mu[1] = 1;
        for p in 2..=BUCKET {
            if !is_composite[p] {
                is_composite[p] = false;
                mu[p] = -1;
                primes.push(p as u32);
            }
            for q in primes
                .iter()
                .take_while(|&&prime| (prime as usize) * p <= BUCKET)
            {
                let val = p * (*q as usize);
                is_composite[val] = true;
                if p % (*q as usize) == 0 {
                    break;
                } else {
                    mu[val] = -mu[p];
                }
            }
            s_mu[p] = (mu[p] as i32) + s_mu[p - 1];
        }
    }
    let mut g_hash: HashMap<usize, i32> = HashMap::new();
    let mut g = |m: usize| -> i32 {
        if let Some(&res) = g_hash.get(&m) {
            res
        } else {
            let mut res = 0;
            for u in 1..=v {
                let mut sub_res =
                    (pow(m as i64, u + 1) * pow((u + 1) as i64, (MOD - 2) as usize)) % MOD;
                sub_res += (pow(2, (MOD-2) as usize) * pow(m as i64, u)) % MOD;
                sub_res %= MOD;
                for k in 2..=u {
                    sub_res += ((((((bernoulli[k] as i64) * (binomial[u][k] as i64)) % MOD)
                        * pow((u - k + 1) as i64, (MOD - 2) as usize))
                        % MOD)
                        * pow(m as i64, (u - k + 1) as usize))
                        % MOD;
                    sub_res %= MOD;
                }
                res += sub_res;
                res %= MOD;
            }
            g_hash.insert(m, res as i32);
            res as i32
        }
    };
    let mut f_hash: HashMap<usize, i32> = HashMap::new();
    let mut f = unsafe { MaybeUninit::zeroed().assume_init() };
    let f_rec = &mut f as *mut dyn FnMut(usize) -> i32;
    f = |m: usize| -> i32 {
        if m <= BUCKET {
            s_mu[m]
        } else if let Some(&res) = f_hash.get(&m) {
            res
        } else {
            let mut res = 1;
            let mut i = m;
            while i > 1 {
                let func = m / i;
                let next_i = m / (func + 1);
                unsafe { res -= (((i - next_i) as i64) * ((*f_rec)(func) as i64)) % MOD };
                res %= MOD;
                i = next_i;
            }
            f_hash.insert(m, res as i32);
            res as i32
        }
    };
    let mut res = 0;
    let bucket = (n as f64).cbrt().powi(2).floor() as usize;
    for k in 1..bucket {
        res += (if k == 1 { v as i64 } else { (pow((k - 1) as i64, (MOD - 2) as usize) * (pow(k as i64, v+1) - 1)
            % MOD)-1 }
            * (f(n / k) as i64))
            % MOD;
        res %= MOD;
    }
    for l in 1..=(n / bucket) {
        res += ((((g(n / l) - g(max(n / (l + 1), bucket-1))) as i64) % MOD) * (f(l) as i64)) % MOD;
        res %= MOD;
    }
    writeln!(sout, "{}", (MOD + res) % MOD).ok();
}
