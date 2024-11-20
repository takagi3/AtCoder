use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u32,
        c: [u32; n],
        r: [u32; n],
    }

    let ans = if c.contains(&t) {
        (0..n)
            .filter(|&i| c[i] == t)
            .max_by_key(|&i| r[i])
            .unwrap()
    } else {
        (0..n)
            .filter(|&i| c[i] == c[0])
            .max_by_key(|&i| r[i])
            .unwrap()
    };

    println!("{}", ans + 1);
}
