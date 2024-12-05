use proconio::input;

fn main() {
    input! {
        a: [u128; 64],
    }

    let ans: u128 = a.iter()
        .enumerate()
        .map(|(i, &x)| x * (1 << i))
        .sum();

    println!("{}", ans);
}
