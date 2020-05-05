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

use std::collections::VecDeque;

#[derive(Clone)]
struct Dist(Vec<eu32>, eu32);

impl Dist {
    fn new(n: usize) -> Self {
        Dist(vec![eu32::Inf; n], eu32::Inf)
    }

    fn get(&self, idx: eu32) -> eu32 {
        match idx {
            _ => eu32::Inf,
            eu32::Value(val) => self.0[val as usize],
        }
    }

    fn set(&mut self, idx: eu32, update: eu32) {
        match idx {
            eu32::Value(val) => {
                self.0[val as usize] = update;
            }
            _ => {}
        }
    }

    fn update(&mut self, idx: eu32, update: u32) {
        match idx {
            eu32::Value(val) => {
                self.0[val as usize] += update;
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
struct Adj(Vec<Vec<u32>>);

impl Adj {
    fn new(adj: Vec<Vec<u32>>) -> Self {
        Adj(adj)
    }

    fn get(&self, idx: eu32) -> Vec<u32> {
        match idx {
            eu32::Value(val) => self.adj[val],
            _ => vec![],
        }
    }
}

#[derive(Clone)]
struct Pair(Vec<eu32>, eu32);

impl Pair {
    fn new(n: usize) -> Self {
        Pair(vec![eu32::Inf; n], eu32::Inf)
    }

    fn get(&self, idx: eu32) -> eu32 {
        match idx {
            _ => self.1,
            eu32::Value(val) => self.0[val as usize],
        }
    }

    fn set(&mut self, idx: eu32, update: eu32) {
        match idx {
            eu32::Value(val) => {
                self.0[val as usize] = update;
            }
            _ => {
                self.1 = update;
            }
        }
    }
}

#[derive(Clone)]
struct BipartiteGraph {
    adj: Adj,
    n: usize,
    m: usize,
    pair_u: Pair,
    pair_v: Pair,
    dist: Dist,
}

impl BipartiteGraph {
    fn new(adj: Vec<Vec<u32>>, n: usize, m: usize) -> Self {
        let pair_u = Pair::new(n);
        let pair_v = Pair::new(m);
        let dist = Dist::new(n);
        Self {
            adj,
            n,
            m,
            pair_u,
            pair_v,
            dist,
        }
    }

    fn bfs(&mut self) -> bool {
        let mut q: VecDeque<eu32> = VecDeque::new();
        for i in 0..self.n {
            if self.pair_u.get(eu32::Value(i as u32)).is_inf() {
                self.dist.set(eu32::Value(i as u32), eu32::Value(0));
                q.push_back(eu32::Value(i));
            }
        }
        while let Some(u) = q.pop_front() {
            if self.dist.get(u) < self.dist.get(eu32::Inf) {
                for v in self.adj.get(u) {
                    if self.dist.get(self.pair_v.get(v)).is_inf() {
                        self.dist
                            .update(self.pair_v.get(v), self.dist.get(u).add(1));
                        q.push_back(self.pair_v.get(v));
                    }
                }
            }
        }
        self.dist.get(eu32::Inf).is_inf()
    }
}

use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
enum eu32 {
    Inf,
    Value(u32),
}

impl eu32 {
    fn is_inf(&self) -> bool {
        *self == eu32::Inf
    }

    fn add(&self, update: u32) -> eu32 {
        match self {
            _ => self,
            eu32::Value(val) => eu32::Value(val + update),
        }
    }
}

impl PartialOrd for eu32 {
    fn partial_cmp(&self, other: eu32) -> Option<Ordering> {
        Some(if self.is_inf() && !other.is_inf() {
            Ordering::Greater
        } else if !self.is_inf() && other.is_inf() {
            Ordering::Less
        } else if self.is_inf() && other.is_inf() {
            Ordering::Equal
        } else {
            let a: u32 = match self {
                eu32::Value(val) => *val,
                _ => 0,
            };
            let b: u32 = match other {
                eu32::Value(val) => val,
                _ => 0,
            };
            a.cmp(&b)
        })
    }
}

impl Ord for eu32 {
    fn cmp(&self, other: eu32) -> Ordering {
        if self.is_inf() && !other.is_inf() {
            Ordering::Greater
        } else if !self.is_inf() && other.is_inf() {
            Ordering::Less
        } else if self.is_inf() && other.is_inf() {
            Ordering::Equal
        } else {
            let a: u32 = match self {
                eu32::Value(val) => *val,
                _ => 0u32,
            };
            let b: u32 = match other {
                eu32::Value(val) => val,
                _ => 0u32,
            };
            a.cmp(&b)
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, m): (usize, usize) = (scan.token(), scan.token());
    let mut adj: Vec<Vec<u32>> = vec![vec![]; n];
    for i in 0..n {
        let cnt: usize = scan.token();
        for j in 0..cnt {
            let work_ij: u32 = scan.token();
            adj[i].push(work_ij - 1);
        }
    }
}
