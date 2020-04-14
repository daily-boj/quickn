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

struct Tree {
    adj: Vec<Vec<usize>>,
    visited: Vec<bool>,
}

impl Tree {
    fn new(adj: Vec<Vec<usize>>) -> Self {
        let visited: Vec<bool> = vec![false;adj.len()];
        Self {
            adj,
            visited,
        }
    }

    fn dfs(&mut self, idx: usize, forbidden: usize) -> usize {
        self.visited[idx] = true;
        let mut res = 0;
        let mut cnt = 0;
        for u in self.adj[idx].clone() {
            if !self.visited[u] && u != forbidden {
                res += self.dfs(u, forbidden);
                cnt += 1;
            }
        }
        if cnt == 0 {
            1
        } else {
            res
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let n: usize = scan.token();
    let mut adj: Vec<Vec<usize>> = vec![vec![];n];
    for i in 0..n {
        let input: i32 = scan.token();
        if input != -1 {
            adj[input as usize].push(i);
            adj[i].push(input as usize);
        }
    }
    let node: usize = scan.token();
    let mut tree = Tree::new(adj.clone());
    if node == 0 && n > 1 {
        writeln!(sout, "{}", tree.dfs(node+1, node)).ok();
    } else {
        writeln!(sout, "{}", tree.dfs(0, node)).ok();
    }
}