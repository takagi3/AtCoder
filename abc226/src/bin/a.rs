use proconio::input;

fn main() {
    input! {
        x: f32,
    }

    let ans: f32 = x.round();

    println!("{}", ans);
}
