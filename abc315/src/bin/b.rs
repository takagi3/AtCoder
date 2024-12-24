use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [isize; m],
    }

    let year: isize = d.iter().sum();
    let mut cnt = (year + 1) / 2;
    let mut ans = (1, 1);
    for (i, &days) in d.iter().enumerate() {
        if cnt <= days {
            ans = (i + 1, cnt as usize);
            break;
        }
        cnt -= days;
    }

    println!("{} {}", ans.0, ans.1);
}
