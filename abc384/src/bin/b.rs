use proconio::input;

fn main() {
    input! {
        n: usize,
        mut r: i64,
        da: [(i32, i64); n],
    }

    for (d, a) in da {
        if (d == 1 && (1600..=2799).contains(&r)) || (d == 2 && (1200..=2399).contains(&r)) {
            r += a;
        }
    }

    println!("{}", r);
}
