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

const MAX_SHIFT: usize = 8_388_608;
const SHIFT: u8 = 23;
const P: i64 = 998_244_353;
const G: i64 = 15311432;
const G_INV: i64 = 469870224;

static mut G_POW: [i32; MAX_SHIFT] = [0; MAX_SHIFT];
static mut G_INV_POW: [i32; MAX_SHIFT] = [0; MAX_SHIFT];

fn fast_pow(a: i64, x: usize) -> i64 {
    let (mut a_t, mut x_t, mut r) = (a, x, 1);
    while x_t != 0 {
        if x_t & 1 == 1 {
            r *= a_t;
            r %= P;
        }
        a_t *= a_t;
        a_t %= P;
        x_t >>= 1;
    }
    r
}

fn ntt(arr: &mut Vec<i32>, shift: u8, inv: bool) {
    let n = arr.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j >= bit {
            j -= bit;
            bit >>= 1;
        }
        j += bit;
        if i < j {
            arr.swap(i, j);
        }
    }
    for l in 1..=shift {
        let m = (1 << l) as i64;
        let omega_m = if inv {
            unsafe { G_INV_POW[(1 << (SHIFT - l)) % MAX_SHIFT] as i64 }
        } else {
            unsafe { G_POW[(1 << (SHIFT - l)) % MAX_SHIFT] as i64 }
        };
        for k in (0..n).step_by(m as usize) {
            let mut omega = 1;
            for j in 0..((m as usize) >> 1) {
                let t = (omega * (arr[k + j + ((m >> 1) as usize)] as i64)) % P;
                arr[k + j + ((m >> 1) as usize)] = (((arr[k + j] as i64) - t) % P) as i32;
                arr[k + j] = (((arr[k + j] as i64) + t) % P) as i32;
                omega = (omega * omega_m) % P;
            }
        }
    }
    if inv {
        let n_inv = fast_pow(n as i64, (P - 2) as usize);
        for i in 0..n {
            arr[i] = (((arr[i] as i64) * n_inv) % P) as i32;
        }
    }
}

use std::cmp::max;

fn mut_polynomial(f: Vec<i32>, g: Vec<i32>) -> Vec<i32> {
    let mut f_cloned = f.clone();
    let mut g_cloned = g.clone();
    let tmp = max(f_cloned.len(), g_cloned.len());
    let mut new_len = 1;
    let mut shift = 0;
    while new_len <= (tmp << 1) {
        new_len <<= 1;
        shift += 1;
    }
    f_cloned.resize(new_len, 0);
    g_cloned.resize(new_len, 0);
    ntt(&mut f_cloned, shift, false);
    ntt(&mut g_cloned, shift, false);
    for i in 0..new_len {
        f_cloned[i] = (((f_cloned[i] as i64) * (g_cloned[i] as i64)) % P) as i32;
    }
    ntt(&mut f_cloned, shift, true);
    let mut idx = new_len-1;
    while idx > 1 && f_cloned[idx] == 0 {
        idx -= 1;
    }
    if idx+1 < new_len {
        f_cloned.drain((idx+1)..);
    }
    f_cloned
}

struct IndexTree {
    data: Vec<Vec<i32>>,
    arr: Vec<i32>,
}

impl IndexTree {
    fn new(d: Vec<u32>, len: usize) -> Self {
        let data: Vec<Vec<i32>> = vec![vec![0]; len << 2];
        let mut arr: Vec<i32> = vec![0; len];
        for i in 0..len {
            arr[i] = -fast_pow(d[i] as i64, (P - 2) as usize) as i32;
        }
        let mut res = Self { data, arr };
        res.init(1, 0, len - 1);
        res
    }

    fn init(&mut self, node: usize, l: usize, r: usize) {
        if l <= r {
            if l == r {
                self.data[node] = vec![1, self.arr[l]];
            } else {
                let mid = (l + r) >> 1;
                self.init(node << 1, l, mid);
                self.init((node << 1) + 1, mid + 1, r);
                self.data[node] = mut_polynomial(
                    self.data[node << 1].clone(),
                    self.data[(node << 1) + 1].clone(),
                );
            }
        }
    }
}

use std::cmp::min;

fn pre_processing() {
    let (mut g, mut g_inv) = (1, 1);
    unsafe {
        G_POW[0] = g as i32;
        G_INV_POW[0] = g_inv as i32;
    }
    for i in 1..MAX_SHIFT {
        g = (((g as i64) * (G as i64)) % P) as i32;
        g_inv = (((g_inv as i64) * (G_INV as i64)) % P) as i32;
        unsafe {
            G_POW[i] = g as i32;
            G_INV_POW[i] = g_inv as i32;
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut d: Vec<u32> = vec![0; n];
    let mut d_min = P as u32;
    for i in 0..n {
        d[i] = scan.token();
        d_min = min(d[i], d_min);
    }
    pre_processing();
    let tree = IndexTree::new(d, n);

    let mut res: i64 = 0;
    // Calculate integeral of probability function P(X=x) 0 to d_min
    for i in 0..tree.data[1].len() {
        let idx = tree.data[1].len() - i - 1;
        res += ((tree.data[1][idx] as i64) * fast_pow((idx + 1) as i64, (P - 2) as usize)) % P;
        res %= P;
        res *= d_min as i64;
        res %= P;
    }

    let mut res2: i64 = 0;
    for i in 0..tree.data[1].len() {
        let idx = tree.data[1].len() - i - 1;
        res2 += tree.data[1][idx] as i64;
        res2 %= P;
        res2 *= d_min as i64;
        res2 %= P;
    }
    writeln!(sout, "{}", (P + res - res2) % P).ok();
}
