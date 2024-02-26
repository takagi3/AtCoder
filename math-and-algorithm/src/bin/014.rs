use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut ans: Vec<u64> = Vec::new();
    let mut rem: u64 = n;

    loop {
        let mut is_end: bool = true;
        for i in 2..=(n as f64).sqrt() as u64 {
            if rem % i == 0 {
                ans.push(i);
                rem /= i;
                is_end = false;
                break;
            }
        }
        if is_end { break; }
    }

    if rem > 1 { ans.push(rem); }

    println!("{}", ans
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
