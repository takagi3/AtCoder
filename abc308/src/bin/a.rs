use proconio::input;

fn main() {
    input! {
        s: [u32; 8],
    }

    let ans: &str = if s.iter().all(|&x| (100..=675).contains(&x) && x % 25 == 0)
        && s.windows(2).all(|w| w[0] <= w[1])
    {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
