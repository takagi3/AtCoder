use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    if let Some(&max) = a.iter().max() {
        let ans = a.iter().filter(|&&x| x != max).max().unwrap_or(&0);
        println!("{}", ans);
    }
}
