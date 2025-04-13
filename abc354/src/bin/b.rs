use proconio::input;

fn main() {
    input! {
        n: usize,
        mut sc: [(String, usize); n],
    }

    let sum: usize = sc.iter().map(|(_, c)| c).sum();
    sc.sort();

    println!("{}", sc[sum % n].0);
}
