use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    match (a, b) {
        (1, 2) | (2, 1) => println!("3"),
        (1, 3) | (3, 1) => println!("2"),
        (2, 3) | (3, 2) => println!("1"),
        _ => println!("-1"),
    }
}