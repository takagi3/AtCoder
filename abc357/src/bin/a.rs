use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: i32,
        h: [i32; n],
    }

    for (i, &v) in h.iter().enumerate() {
        m -= v;
        if m < 0 {
            println!("{}", i);
            return;
        }
    }

    println!("{}", n);
}
