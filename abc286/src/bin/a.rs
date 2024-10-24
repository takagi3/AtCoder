use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        a: [u32; n],
    }

    let mut b: Vec<u32> = vec![0; n];
    for i in 0..n {
        if p - 1 <= i && i <= q - 1 {
            b[i] = a[r + i - p]
        } else if r - 1 <= i && i <= s - 1 {
            b[i] = a[p + i - r]
        } else {
            b[i] = a[i]
        }
    }

    println!("{}", b.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
