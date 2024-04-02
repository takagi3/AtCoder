use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut ans: Vec<u32> = vec![];
    for i in 2..=n {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            ans.push(i)
        }
    }

    println!("{}", ans.iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
