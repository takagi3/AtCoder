use proconio::input;

fn main() {
    input! {
        n: f32,
    }

    let x: u32 = n as u32;
    let y: u32 = (n * 10.0) as u32 % 10;
    let mut ans: String = x.to_string();
    if y <= 2 {
        ans += "-";
    } else if 7 <= y {
        ans += "+";
    }

    println!("{}", ans);
}
