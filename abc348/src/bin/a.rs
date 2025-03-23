use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans: String = (1..=n)
        .map(|i| if i % 3 == 0 { 'x' } else { 'o' })
        .collect();

    println!("{}", ans);
}
