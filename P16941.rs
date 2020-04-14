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

fn pow_mod(a: i64, x: usize, p: i64) -> i64 {
    let (mut r, mut a_t, mut x_t) = (1, a, x);
    while x_t != 0 {
        if x_t & 1 == 1 {
            r *= a_t;
            r %= p;
        }
        a_t *= a_t;
        a_t %= p;
        x_t >>= 1;
    }
    r
}

use std::collections::HashMap;
use std::mem::MaybeUninit;

const LIMIT: usize = 500_000;
const MAX_Q: usize = 1_001;
const MAX_SQRT: usize = 31_624;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (p, q): (i64, i64) = (scan.token(), scan.token());
    if p < (LIMIT as i64) {
        // Naïve implementation
        let mut is_composite: Box<[bool;LIMIT]> = Box::new([false;LIMIT]);
        let mut primes: Vec<u32> = Vec::new();
        for i in 2..=(p as u32) {
            if !is_composite[i as usize] {
                primes.push(i);
            }
            for j in primes.iter().take_while(|&it| it * i <= (p as u32)) {
                let val = i * j;
                is_composite[val as usize] = true;
                if i % j == 0 {
                    break;
                }
            }
        }
        let res = primes.iter().fold(0i32, |mut sum, &val| {
            sum = (((sum as i64) + pow_mod(val as i64, q as usize, p)) % p) as i32;
            sum
        });
        writeln!(sout, "{}", res).ok();
    } else {
        // Efficient implementation
        // Calculate binomial coefficient by recurrence relation \binom{n}{k} = \binom{n-1}{k-1} + \binom{n-1}{k}
        let mut dp: Box<[[i32;MAX_Q];MAX_Q]> = Box::new([[0;MAX_Q];MAX_Q]);
        for i in 1..=(q as usize) {
            dp[i][0] = 1;
            dp[i][i] = 1;
            for j in 1..(i as usize) {
                dp[i][j] = (((dp[i - 1][j] as i64) + (dp[i - 1][j - 1] as i64)) % p) as i32;
            }
        }

        // Calculate bernoulli number B_{m}^{+} by recurrence relation B_{m}^{+} = 1 - \sum_{k=0}^{m-1} \frac{\binom{m}{k} B_{k}^{+}}{m-k+1}
        let mut bernoulli: Box<[i32;MAX_Q]> = Box::new([0;MAX_Q]);
        bernoulli[0] = 1;
        for m in 1..=(q as usize) {
            let mut res: i32 = 1;
            for k in 0..m {
                res = (((res as i64)
                    - ((((dp[m][k] as i64) * (bernoulli[k] as i64) % p)
                        * pow_mod((m - k + 1) as i64, (p - 2) as usize, p))
                        % p))
                    % p) as i32;
            }
            bernoulli[m] = ((p+(res as i64))%p) as i32;
        }

        // Calculate \sum_{k=1}^{f} k^q by faulhaber's formula
        let mut hash1: HashMap<u32, i32> = HashMap::new();
        let mut i = ((p as usize)/LIMIT)+1;
        let tmp = pow_mod((q + 1) as i64, (p - 2) as usize, p);
        let tmp2 = pow_mod(2, (p - 2) as usize, p);
        // By harmonic lemma, it costs \O(\sqrt{p}q\log{q})
        while i > 0 {
            let f = (p as usize) / i;
            if q != 0 {
                let mut sum = ((((pow_mod(f as i64, (q + 1) as usize, p) * tmp) % p)
                    + ((pow_mod(f as i64, q as usize, p) * tmp2) % p))
                    % p) as i32;
                for k in 2..=(q as usize) {
                    sum = (((sum as i64)
                        + (((dp[q as usize][k] as i64)
                            * pow_mod(((q as usize) - k + 1) as i64, (p - 2) as usize, p))
                            % p)
                            * (((bernoulli[k] as i64)
                                * pow_mod(f as i64, (q as usize) - k + 1, p))
                                % p))
                        % p) as i32;
                }
                hash1.insert(f as u32, sum);
            } else {
                hash1.insert(f as u32, f as i32);
            }
            i = (p as usize) / (f + 1);
        }

        let mut psum: i32 = 0;
        for j in 1..=LIMIT {
            psum = (((psum as i64) + pow_mod(j as i64, q as usize, p)) % p) as i32;
            hash1.insert(j as u32, psum);
        }

        let sqrt_p = (p as f64).sqrt().floor() as u32;
        let mut is_composite: Box<[bool;MAX_SQRT]> = Box::new([false;MAX_SQRT]);
        let mut primes: Vec<u32> = Vec::new();
        for i in 2..=sqrt_p {
            if !is_composite[i as usize] {
                primes.push(i);
            }
            for j in primes.iter().take_while(|&it| it * i <= sqrt_p) {
                let val = i * j;
                is_composite[val as usize] = true;
                if i % j == 0 {
                    break;
                }
            }
        }
        let mut primes_pow: Vec<i32> = vec![0; primes.len()];
        for i in 0..primes_pow.len() {
            primes_pow[i] = pow_mod(primes[i] as i64, q as usize, p) as i32;
        }
    
        // Calculate prime counting \phi-like function
        // \phi(x,a) = \sum\nolimits_{1 \leq k \leq p \wedge p_1,...,p_a \nmid k} k^q
        // by using recurrence \phi(x,a)
        //                     = \phi(x,a-1) - {p_a}^{q}\phi(\lfloor\frac{x}{p_a}\rfloor,a-1)
        let mut phi = unsafe { MaybeUninit::zeroed().assume_init() };
        let phi_rec = &mut phi as *mut dyn FnMut(u32, u32) -> i32;
        let mut hash2: HashMap<(u32, u32), i32> = HashMap::new();
        phi = |x: u32, a: u32| -> i32 {
            if x == 0 {
                0
            } else if x == 1 {
                1
            } else if a == 0 {
                hash1[&x]
            } else if let Some(&res) = hash2.get(&(x, a)) {
                res
            } else if a >= 1 && x < primes[(a as usize) - 1] {
                let res = unsafe { (*phi_rec)(x, a - 1) };
                hash2.insert((x, a), res);
                res
            } else {
                let res = unsafe {
                    (((*phi_rec)(x, a - 1) as i64)
                        - (((primes_pow[(a as usize) - 1] as i64)
                            * ((*phi_rec)(x / primes[(a as usize) - 1], a - 1) as i64))
                            % p))
                        % p
                } as i32;
                hash2.insert((x, a), res);
                res
            }
        };

        let mut res = ((p + (phi(p as u32, primes.len() as u32) as i64)) % p) as i32;
        res = ((p
            + (((res as i64) - 1
                + (primes_pow.iter().fold(0i32, |mut sum, val| {
                    sum = (((sum as i64) + (*val as i64)) % p) as i32;
                    sum
                }) as i64))
                % p))
            % p) as i32;
        writeln!(sout, "{}", res).ok();
    }
}
