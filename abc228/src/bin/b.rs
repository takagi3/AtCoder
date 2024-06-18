use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut secret: Vec<bool> = vec![false; n + 1];
    secret[x] = true;
    let mut next: usize = a[x - 1];
    loop {
        if secret[next] {
            break;
        } else {
            secret[next] = true;
            next = a[next - 1];
        }
    }
    let mut ans: u32 = 0;
    for i in 0..=n {
        if secret[i] {
            ans += 1
        }
    }

    println!("{}", ans);
}
