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

const MAX: usize = 4000001;

static mut GROUPS: usize = 0;
static mut PI: [usize;MAX] = [0;MAX];
static mut DEPTH: [usize;MAX] = [0;MAX];

fn init(k: usize) {
    // Assume it's safe
    unsafe {
        for i in 0..=k {
            PI[i] = i;
        }
    }
}

fn root(n: usize) -> usize {
    // Assume it's safe
    unsafe {
        if PI[n] == n {
            n
        } else {
            let res = root(PI[n]);
            PI[n] = res;
            res
        }
    }
}

fn union(n: usize, m: usize) {
    // Assume it's safe
    unsafe {
        let (n_t, m_t) = (root(n), root(m));
        if n_t != m_t {
            if DEPTH[n_t] < DEPTH[m_t] {
                PI[n_t] = m_t;
            } else if DEPTH[n_t] > DEPTH[m_t] {
                PI[m_t] = n_t;
            } else {
                PI[m_t] = n_t;
                DEPTH[n_t] += 1;
            }
            GROUPS -= 1;
        }
    }
}

use std::collections::VecDeque;

const DX: [i32;4] = [1,-1,0,0];
const DY: [i32;4] = [0,0,1,-1];
const INF: usize = 2000000000;

fn main() {
    let (stdio, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (UnsafeScanner::new(stdio.lock()), BufWriter::new(stdout.lock()));
    let (n, k): (usize, usize) = (scan.token(), scan.token());
    let mut board: Vec<Vec<(usize,usize)>> = vec![vec![(0,INF);n];n];
    let mut visited: Vec<Vec<bool>> = vec![vec![false;n];n];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut cnt = 0;
    for row in 0..n {
        for column in 0..n {
            board[row][column].0 = cnt;
            cnt += 1;
        }
    }
    for _i in 1..=k {
        let (x, y): (usize, usize) = (scan.token(), scan.token());
        board[x-1][y-1].1 = 0;
        q.push_back((x-1, y-1));
    }
    init(cnt);
    let mut res = n<<1;
    for i in 0..=(n<<1) {
        while let Some((x_i, y_i)) = q.pop_front() {
            if board[x_i][y_i].1 != i {
                q.push_back((x_i, y_i));
                break;
            }
            visited[x_i][y_i] = true;
            unsafe { GROUPS += 1; }
            for j in 0..4 {
                let (xp, yp) = ((x_i as i32) + DX[j], (y_i as i32) + DY[j]);
                if xp < 0 || yp < 0 || xp >= (n as i32) || yp >= (n as i32) {
                    continue;
                }
                if visited[xp as usize][yp as usize] {
                    union(board[xp as usize][yp as usize].0, board[x_i][y_i].0);
                } else if board[xp as usize][yp as usize].1 == INF {
                    board[xp as usize][yp as usize].1 = board[x_i][y_i].1 + 1;
                    q.push_back((xp as usize, yp as usize));
                }
            }
        }
        if unsafe { GROUPS } == 1 {
            res = i;
            break;
        }
    }
    writeln!(sout, "{}", res).ok();
}