use proconio::input;

fn main() {
    input! {
        b: u64,
    }

    let mut ans = -1;
    for i in 1u64.. {
        match i.checked_pow(i as u32) {
            Some(power) if power == b => {
                ans = i as i64;
                break;
            }
            Some(power) if power > b => break,
            None => break,
            _ => {}
        }
    }

    println!("{}", ans);
}
