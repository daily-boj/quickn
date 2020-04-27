/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

use std::arch::x86_64::_rdrand64_step;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::mem::{swap, MaybeUninit};

const MOD: i64 = 1_000_000_007;
const K: usize = 1_000_000;

static mut F: [i64; (K) + 1] = [0; (K) + 1];
static mut IS_COMPOSITE: [bool; (K) + 1] = [false; (K) + 1];
static mut MU_L: [i64; (K) + 1] = [0; (K) + 1];

fn fast_pow(a: i64, x: u64) -> i64 {
    let mut r = 1;
    let mut a_t = a;
    let mut x_t = x;
    while x_t != 0 {
        if (x_t & 1) == 1 {
            r = (r * a_t) % MOD;
        }
        a_t = (a_t * a_t) % MOD;
        x_t >>= 1;
    }
    r
}

// Caclulate s_(mu_l) (prefix sum of mu_l) by 0..=K
// Complexity of linear sieve is O(K)
fn pre1(l: u64) {
    let mut primes: Vec<u64> = Vec::new();
    let mut f_i = 1; // prefix sum of mu_l
    unsafe {
        F[1] = 1;
        MU_L[1] = 1;
    }
    for p in 2..=(K) {
        if unsafe { !IS_COMPOSITE[p] } {
            primes.push(p as u64);
            unsafe {
                MU_L[p] = if l % (p as u64) == 0 { 0 } else { -1 };
            }
        }
        for q in primes
            .iter()
            .take_while(|&n| n * (p as u64) <= ((K) as u64))
        {
            let v = q * (p as u64);
            unsafe {
                IS_COMPOSITE[v as usize] = true;
            }
            if (p as u64) % q == 0 {
                break;
            } else {
                unsafe {
                    MU_L[v as usize] = MU_L[p] * MU_L[*q as usize];
                }
            }
        }
        f_i += unsafe { MU_L[p] };
        unsafe {
            F[p] = f_i;
        }
    }
}

// Calculate s_(mu_l*1) (prefix sum of mu_l*1)
// Complexity of pre2 procedure is O(n) or O(sqrt(l))
fn pre2(l: u64, n: u64) -> Vec<u64> {
    let mut func = unsafe { MaybeUninit::zeroed().assume_init() };
    let func_rec = &mut func as *mut dyn FnMut(u64);
    let factors = factorization(l);
    let mut res: BTreeSet<u64> = BTreeSet::new();
    func = |m: u64| {
        res.insert(m);
        for &(factor, _) in &factors {
            let t = m * factor;
            if t <= n && res.get(&t).is_none() {
                unsafe { (*func_rec)(t) };
            }
        }
    };
    func(1);
    res.into_iter().collect()
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

const PRIMES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

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
    factors
        .iter()
        .map(|&factor| (factor, *hash.get(&factor).unwrap()))
        .collect()
}

fn mu(d: u64) -> i8 {
    if d == 1 {
        1
    } else if d % 4 == 0 {
        0
    } else {
        let factors = factorization(d);
        let mut is_zero = false;
        for &(_, exp) in &factors {
            if exp > 1 {
                is_zero = true;
                break;
            }
        }
        if is_zero {
            0
        } else if factors.len() % 2 == 0 {
            1
        } else {
            -1
        }
    }
}

fn main() {
    let mut sout = BufWriter::new(io::stdout());
    let mut sin = BufReader::new(io::stdin());
    let mut buf = String::new();
    sin.read_line(&mut buf).unwrap();
    let buf_wp: Vec<u64> = buf
        .split_whitespace()
        .map(|it| it.trim().parse::<u64>().unwrap())
        .collect();
    let (n, l, k) = (buf_wp[0], buf_wp[1], buf_wp[2]);
    pre1(l);
    let factors_l = pre2(l, n);
    let s_fg = |m: u64| -> u64 {
        let res = factors_l.binary_search(&m);
        if res.is_ok() {
            (res.unwrap() as u64) + 1
        } else {
            res.unwrap_err() as u64
        }
    };
    let mut f_hash: HashMap<u64, i64> = HashMap::new();
    // O(m^(2/3))
    let mut s_f = unsafe { MaybeUninit::zeroed().assume_init() };
    let y = { &mut s_f as *mut dyn FnMut(u64) -> i64 };
    s_f = |m: u64| -> i64 {
        if m <= (K as u64) {
            unsafe { F[m as usize] }
        } else {
            if let Some(&res) = f_hash.get(&m) {
                res
            } else {
                let mut res = s_fg(m) as i64;
                let mut i = m;
                while i > 1 {
                    let j = m / i;
                    let next_i = m / (j + 1);
                    unsafe {
                        res -= ((i - next_i) as i64) * (*y)(j);
                    }
                    i = next_i;
                }
                f_hash.insert(m, res);
                res
            }
        }
    };
    let mut res: i64 = 0;
    let mut i = n;
    while i > 0 {
        let j = n / i;
        let next_i = n / (j + 1);
        res += ((s_f(i) - s_f(next_i)) * fast_pow(j as i64, k)) % MOD;
        res %= MOD;
        i = next_i;
    }
    writeln!(sout, "{}", (MOD + res * (mu(l) as i64)) % MOD).unwrap();
}
