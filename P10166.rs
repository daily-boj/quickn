/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

use std::io::{self, BufReader, BufWriter, BufRead, Write};

fn main() {
    let mut sout = BufWriter::new(io::stdout());
    let mut sin = BufReader::new(io::stdin());
    let mut buf = String::new();
    sin.read_line(&mut buf).unwrap();
    let buf_wp: Vec<u32> = buf.split_whitespace().map(|it| it.trim().parse::<u32>().unwrap()).collect();
    let (d1, d2) = (buf_wp[0], buf_wp[1]);
    let mut cnt: Vec<u64> = Vec::new();
    let mut cnt2: Vec<u64> = Vec::new();
    cnt.resize(d2 as usize,0);
    cnt2.resize(d2 as usize,0);
    for p in 2..=d2 {
        cnt[(p - 1) as usize] = p as u64-1;
        cnt2[(p - 1) as usize] = p as u64-1;
    }
    let mut sum: u64 = 0;
    for p in 2..=d2 {
        if p >= d1 {
            sum += cnt[(p - 1) as usize];
        }
        let mut chance = false;
        for q in ((p << 1)..=d2).step_by(p as usize) {
            cnt2[(q - 1) as usize] -= cnt2[(p - 1) as usize];
            if (q >= d1 && chance) || q < d1 || p >= d1 {
                cnt[(q - 1) as usize] -= cnt2[(p - 1) as usize];
            }
            if q >= d1 { chance = true; }
        }
    }
    writeln!(sout, "{}", sum+1).unwrap();
}
 