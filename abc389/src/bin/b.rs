use proconio::input;

fn main() {
    input! {
        x: u64,
    }

    let ans = (1..=20)
        .scan(1u64, |fact, i| {
            *fact *= i;
            Some((*fact, i))
        })
        .find(|&(fact, _)| fact == x)
        .map_or(0, |(_, i)| i);

    println!("{}", ans);
}
