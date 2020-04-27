/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

fn fast_pow(a: u64, x: u64, p: u64) -> u64 {
    let mut r = 1;
    let mut a_t = a;
    let mut x_t = x;
    while x_t != 0 {
        if (x_t & 1) == 1 { r = (r*a_t) % p; }
        a_t = (a_t*a_t) % p;
        x_t >>= 1;
    }
    r
}