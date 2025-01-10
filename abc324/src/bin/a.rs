use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let ans = if a.windows(2).all(|w| w[0] == w[1]) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
