use proconio::input;

fn main() {
    input! {
        a: [usize; 10],
    }

    let mut ans: usize = 0;
    for _ in 0..3 {
        ans = a[ans]
    }

    println!("{}", ans);
}
