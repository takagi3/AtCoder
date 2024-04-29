use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans: Vec<u32> = vec![];
    // for i in 2..=n {
    //     let mut is_prime = true;
    //     for j in 2..i {
    //         if i % j == 0 {
    //             is_prime = false;
    //             break;
    //         }
    //     }
    //     if is_prime {
    //         ans.push(i)
    //     }
    // }

    let mut prime: Vec<bool> = vec![true; n + 1];
    for i in 2..n.sqrt() + 1 {
        if prime[i] {
            for j in (2 * i..=n).step_by(i) {
                prime[j] = false
            }
        }
    }

    for i in 2..=n {
        if prime[i] {
            ans.push(i as u32);
        }
    }

    println!("{}", ans.iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
