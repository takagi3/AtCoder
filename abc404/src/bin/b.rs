use proconio::{input, marker::Chars};

#[inline]
fn map_idx(r: usize, i: usize, j: usize, n: usize) -> (usize, usize) {
    match r {
        0 => (i, j),
        1 => (n - 1 - j, i),
        2 => (n - 1 - i, n - 1 - j),
        _ => (j, n - 1 - i),
    }
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }

    let mut best = usize::MAX;

    for r in 0..4 {
        let mut diff = 0usize;
        for i in 0..n {
            let ti = &t[i];
            for j in 0..n {
                let (si, sj) = map_idx(r, i, j, n);
                diff += (s[si][sj] != ti[j]) as usize;
            }
        }
        best = best.min(r + diff);
    }

    println!("{}", best);
}
