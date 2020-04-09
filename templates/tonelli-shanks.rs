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

fn Tonelli_Shanks(n: u64, p: u64) -> Result<u64, ()> {
    // O((logp)^2)
    if fast_pow(n, (p-1)>>1, p) != 1 {
        Err(())
    } else {
        let (mut q, s) = (p-1, 0);
        while q % 2 == 0 {
            q >>= 1;
            s += 1;
        }
        // g(p) <= (logp)^2 and g(p) <= sqrt(p) + 1
        let mut z = 0;
        for i in 1..p {
            if fast_pow(i, (p-1)>>1, p) == p-1 {
                z = i;
                break;
            }
        }
        let (mut m, mut c, mut t, mut r) = (s, fast_pow(z, q, p), fast_pow(n, q, p), fast_pow(n, (q+1)>>1, p));
        loop {
            if t == 0 { r = 0; break; }
            if t == 1 { break; }
            let mut i = 1;
            let mut ev = 2;
            while i < m {
                if fast_pow(t, ev, p) == 1 {
                    break;
                }
                i += 1;
                ev <<= 1;
            }
            let b = fast_pow(c, 1 << (m-i-1), p);
            m = i;
            c = (b*b) % p;
            t = (t*((b*b) % p)) % p;
            r = (r*b) % p;
        }
        Ok(r)
    }
}