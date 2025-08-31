use proconio::input;

fn main() {
    input! {
        mut a: [i32; 5],
    }

    let mut cnt: u32 = 0;
    for i in 0..4 {
        if a[i] > a[i + 1] {
            (a[i], a[i + 1]) = (a[i + 1], a[i]);
            cnt += 1;
        }
    }

    println!("{}", if cnt == 1 { "Yes" } else { "No" });
}
