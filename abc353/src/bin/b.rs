use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut iter = a.iter().peekable();
    let mut car = 0;
    while let Some(&current) = iter.next() {
        car += current;
        if let Some(&&next) = iter.peek() {
            if car + next > k {
                ans += 1;
                car = 0;
            }
        }
    }

    println!("{}", ans + 1);
}
