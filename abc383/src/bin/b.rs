use proconio::input;

fn main() {
    input! {
        h: usize,
        _w: usize,
        d: usize,
        s: [String; h],
    }

    let free: Vec<(usize, usize)> = s
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars().enumerate().filter_map(
                move |(j, c)| {
                    if c == '.' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .collect();
    let mut ans = 0;
    let n = free.len();
    for a in 0..n {
        for b in a + 1..n {
            let (i1, j1) = free[a];
            let (i2, j2) = free[b];

            let cnt = free
                .iter()
                .filter(|&&(i, j)| {
                    let d1 = (i1.max(i) - i1.min(i)) + (j1.max(j) - j1.min(j));
                    let d2 = (i2.max(i) - i2.min(i)) + (j2.max(j) - j2.min(j));
                    d1 <= d || d2 <= d
                })
                .count();
            ans = ans.max(cnt);
        }
    }

    println!("{}", ans);
}
