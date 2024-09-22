use proconio::input;

fn main() {
    input! {
        n: u32
    }

    let x: String = format!("{:x}", n).to_uppercase();
    let ans: String = format!("{x:0>2}");

    println!("{}", ans);
}
