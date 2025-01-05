use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i32,
        mut a: [i32; n - 1],
    }

    let mut ans = -1;
    for i in 0..=100 {
        let mut extended = a.clone();
        extended.push(i);
        extended.sort_unstable();
        let sum: i32 = extended[1..n - 1].iter().sum();
        if sum >= x {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}
