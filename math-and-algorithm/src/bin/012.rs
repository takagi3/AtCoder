use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
