use proconio::input;

fn main() {
    input! {
        mut n: u64,
    }

    let mut ans: Vec<u64> = vec![];
    loop {
        let mut is_end: bool = true;
        for i in 2..=(n as f64).sqrt() as u64 {
            if n % i == 0 {
                ans.push(i);
                n /= i;
                is_end = false;
                break;
            }
        }
        if is_end {
            ans.push(n);
            break;
        }
    }

    println!("{}", ans.iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
