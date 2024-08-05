use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32; n],
        b: [u32; k],
    }

    let mut max: u32 = 0;
    for i in 0..n {
        if a[i] > max {
            max = a[i]
        }
    }
    let mut ans: &str = "No";
    for i in 0..n {
        if a[i] == max && b.contains(&(i as u32 + 1)) {
            ans = "Yes"
        }
    }

    println!("{}", ans);
}
