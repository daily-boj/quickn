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

#[derive(Clone)]
struct Tree {
    adj: Vec<Vec<u32>>,
    res: Vec<u32>,
    res2: Vec<u32>,
}

impl Tree {
    fn new(adj: Vec<Vec<u32>>) -> Self {
        let res: Vec<u32> = vec![0;adj.len()-1];
        let res2: Vec<u32> = vec![0;adj.len()-1];
        Self {
            adj,
            res,
            res2,
        }
    }

    fn dfs(&mut self, idx: u32) -> u32 {
        if self.adj[idx as usize].len() == 0 {
            self.res[(idx as usize)-1] = 1;
            self.res2[(idx as usize)-1] = idx;
            return idx;
        }
        let mut res: u32 = 0;
        for u in self.adj[idx as usize].clone() {
            self.res2[(idx as usize)-1] = self.dfs(u);
            res += self.res[(u as usize)-1];
        }
        self.res[(idx as usize)-1] = res;
        self.res2[(idx as usize)-1]
    }
}

const MAX: usize = 3_000_000;

struct UnionFind {
    pi: Box<[u32;MAX]>,
    cnt: Box<[u32;MAX]>,
    depth: Box<[u32;MAX]>,
}

use std::alloc::{alloc, Layout};

impl UnionFind {
    fn new(k: usize) -> Self {
        let layout1 = Layout::new::<[u32; MAX]>();
        let layout2 = Layout::new::<[u32; MAX]>();
        let layout3 = Layout::new::<[u32; MAX]>();
        let mut pi = unsafe {
            let ptr = alloc(layout1) as *mut [u32; MAX];
            Box::from_raw(ptr)
        };
        let mut cnt = unsafe {
            let ptr = alloc(layout2) as *mut [u32; MAX];
            Box::from_raw(ptr)
        };
        let mut depth = unsafe {
            let ptr = alloc(layout3) as *mut [u32; MAX];
            Box::from_raw(ptr)
        };
        for i in 0..k {
            pi[i] = i as u32;
            cnt[i] = 1;
            depth[i] = 0;
        }
        Self {
            pi,
            cnt,
            depth,
        }
    }

    fn root(&self, n: u32) -> u32 {
        let mut res = n;
        while self.pi[res as usize] != res {
            res = self.pi[res as usize];
        }
        res
    }
    
    fn union(&mut self, n: u32, m: u32) {
        let (n_t, m_t) = (self.root(n), self.root(m));
        if n_t != m_t {
            if self.depth[n_t as usize] < self.depth[m_t as usize] {
                self.pi[n_t as usize] = m_t as u32;
                self.cnt[m_t as usize] += self.cnt[n_t as usize];
            } else if self.depth[n_t as usize] > self.depth[m_t as usize] {
                self.pi[m_t as usize] = n_t as u32;
                self.cnt[n_t as usize] += self.cnt[m_t as usize];
            } else {
                self.pi[m_t as usize] = n_t as u32;
                self.depth[n_t as usize] += 1;
                self.cnt[n_t as usize] += self.cnt[m_t as usize];
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let (n1, n2, k): (usize, usize, usize) = (scan.token(), scan.token(), scan.token());
    let mut adj1: Vec<Vec<u32>> = vec![vec![];n1+1];
    let mut adj2: Vec<Vec<u32>> = vec![vec![];n2+1];
    for i in 1..=n1 {
        let u: usize = scan.token();
        adj1[u].push(i as u32);
    }
    for i in 1..=n2 {
        let u: usize = scan.token();
        adj2[u].push(i as u32);
    }
    let mut uf: UnionFind = UnionFind::new(k);
    let (mut tree1, mut tree2) = (Tree::new(adj1.clone()), Tree::new(adj2.clone()));
    tree1.dfs(adj1[0][0]);
    tree2.dfs(adj2[0][0]);
    let mut t1: Vec<(u32, u32, u32)> = tree1.res.iter().enumerate().map(|(i, set)| (0, i as u32, *set)).collect();
    let mut t2: Vec<(u32, u32, u32)> = tree2.res.iter().enumerate().map(|(i, set)| (1, i as u32, *set)).collect();
    t1.append(&mut t2);
    t1.sort_by(|(_i, _j, set), (_i2, _j2, set2)| set.cmp(&set2));
    let mut res = true;
    for (kind, idx, set) in t1 {
        match kind {
            0 => {
                if adj1[(idx as usize)+1].len() == 0 {
                    continue;
                }
                let t = tree1.res2[(adj1[(idx as usize)+1][0] as usize)-1];
                for &u in &adj1[(idx as usize)+1] {
                    let v = tree1.res2[(u as usize)-1];
                    uf.union(t-1, v-1);
                }
                if uf.cnt[uf.root(t-1) as usize] > set {
                    res = false;
                    break;
                }
            },
            _ => {
                if adj2[(idx as usize)+1].len() == 0 {
                    continue;
                }
                let t = tree2.res2[(adj2[(idx as usize)+1][0] as usize)-1];
                for &u in &adj2[(idx as usize)+1] {
                    let v = tree2.res2[(u as usize)-1];
                    uf.union(t-1, v-1);
                }
                if uf.cnt[uf.root(t-1) as usize] > set {
                    res = false;
                    break;
                }
            }
        }
    }
    if res {
        writeln!(sout, "YES").ok();
    } else {
        writeln!(sout, "NO").ok();
    }
}