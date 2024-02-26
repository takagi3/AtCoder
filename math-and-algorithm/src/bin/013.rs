use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut ans: Vec<u64> = Vec::new();
    for i in 1..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            ans.push(i);
            if i * i != n {
                ans.push(n / i);
            }
        }
    }

    println!("{}", ans
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
    );
}
