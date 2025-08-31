use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let ok = if n <= 2 {
        true
    } else {
        a.windows(3).all(|w| {
            let (x, y, z) = (w[0] as i128, w[1] as i128, w[2] as i128);
            x * z == y * y
        })
    };

    println!("{}", if ok { "Yes" } else { "No" });
}
