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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let (n, m): (i32, i32) = (scan.token(), scan.token());
    let mut res: u64 = 1 + if n & 1 != 1 { m as u64 } else { 0 };
    for p in 1..=m {
        // We may use sturm's theorem and get sign variations
        // By using,
        // $p_{0} = x^{n} + px + q$
        // $p_{1} = nx^{n-1} + p$
        // $p_{2} = p + \frac{(-1)^{n} n^{n+1} q^{n}}{(1-n)^{n} p^{n}}
        if n & 1 == 1 {
            // For $-\infty$,
            // $p_{0}$ => (-)
            // $p_{1}$ => (+)
            // $p_{2}$ => ?
            // Case #1: $p_{2} > 0 \wedge p > 0 \wedge \frac{n^{n+1} q^{n}}{(1-n)^{n} p^{n}} < 0$
            // $\Rightarrow \frac{q^{n}}{p^{n}} > 0$
            // Then, $q > 0$
            // There exists $q > 0$ that satisfies conditions
            res += m as u64;
            // Case #2: $p_{2} > 0 \wedge p > 0 \wedge p >= \frac{n^{n+1} q^{n}}{(1-n)^{n} p^{n}}$
            // $\Rightarrow 1 >= \frac{n^{n+1} q^{n}}{(1-n)^{n} p^{n+1}}$
            // $\Rightarrow (1-n)^{n} p^{n+1} >= n^{n+1} q^{n}$
            // $\Rightarrow \frac{(1-n)^{n} p^{n+1}}{n^{n+1}} >= q^{n}$
            // $\Rightarrow \frac{(1-n) p^{\frac{1}{n}+1}}{n^{\frac{1}{n}+1}} >= q$
            let upper = (((1-n) as f64)*((p as f64)/(n as f64)).powf(1.0 + 1.0/(n as f64))).floor() as i64;
            res += ((m as i64) + 1 + upper) as u64;
            // Case #3: $p_{2} > 0 \wedge p < 0 \wedge \frac{n^{n+1} q^{n}}{(1-n)^{n} p^{n}} < 0$
        }
    }
}