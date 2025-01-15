use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    let ans = if -3 <= y - x && y - x <= 2 {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
