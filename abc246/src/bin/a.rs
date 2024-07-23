use proconio::input;

fn main() {
    input! {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
    }

    let x: i32;
    let y: i32;

    if x1 == x2 {
        x = x3
    } else if x1 == x3 {
        x = x2
    } else {
        x = x1
    }

    if y1 == y2 {
        y = y3
    } else if y1 == y3 {
        y = y2
    } else {
        y = y1
    }

    println!("{} {}", x, y);
}
