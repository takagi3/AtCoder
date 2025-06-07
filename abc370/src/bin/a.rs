use proconio::input;

fn main() {
    input! {
        (l, r): (u32, u32)
    }

    let ans = match (l, r) {
        (1, 0) => "Yes",
        (0, 1) => "No",
        _ => "Invalid",
    };

    println!("{}", ans);
}
