/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

extern crate core;

use std::collections::HashMap;
use core::arch::x86_64::_rdrand64_step;
use std::mem::swap;
use std::cmp::{min, max};

const PRIMES: [u64;12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

fn fast_gcd(n: u64, m: u64) -> u64 {
    if n == 0 || m == 0 { return max(n, m); }
    let mut n_t = max(n, m);
    let mut m_t = min(n, m);
    while m_t != 0 {
        n_t %= m_t;
        swap(&mut n_t, &mut m_t);
    }
    n_t
}

fn g(x: u64, n: u64) -> u64 {
    ((n as u128+(x as u128*x as u128)-1) % (n as u128)) as u64
}

fn pow_mod(a: u128, x: u128, n: u128) -> u128 {
    let mut a_t = a;
    let mut x_t = x;
    let mut r = 1;
    while x_t != 0 {
        if (x_t & 1) == 1 { r = (r*a_t) % n; }
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
    if x == 1 || x == n-1 { return false; }
    for _i in 0..(t-1) {
        x = ((x as u128*x as u128) % (n as u128)) as u64;
        if x == n-1 {
            return false;
        }
    }
    return true;
}

fn miller_rabin(n: u64) -> bool {
    for p in &PRIMES {
        if *p >= n { break; }
        if witness(*p, n) { return true; }
    }
    return false;
}

fn pollard_rho(n: u64) -> u64 {
    let mut i = 1;
    let mut x = 0;
    unsafe { _rdrand64_step(&mut x); }
    x = x%n;
    let mut y = x;
    let mut k = 2;
    let mut d: u64;
    loop {
        i += 1;
        x = g(x, n);
        d = fast_gcd((y as i64-x as i64).abs() as u64, n);
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

fn factorization(m: u64) -> Vec<(u64,u16)> {
    let mut n = m;
    let mut hash: HashMap<u64,u16> = HashMap::new();
    let mut factors: Vec<u64> = Vec::new();
    while n % 2 == 0 {
        if let Some(&res) = hash.get(&2) {
            hash.insert(2, res+1);
        } else {
            factors.push(2);
        }
        n >>= 1;
    }
    while n != 1 {
        if !miller_rabin(n) { 
            if let Some(&res) = hash.get(&n) {
                hash.insert(n, res+1);
            } else {
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
            hash.insert(x, res+1);
        } else {
            factors.push(x);
        }
    }
    let mut res: Vec<(u64,u16)> = factors.iter().map(|&factor| (factor,0)).collect();
    for i in 0..factors.len() {
        res[i] = (res[i].0, *hash.get(&res[i].0).unwrap());
    }
    res
}