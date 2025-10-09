use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut auth = false;
    let mut ans = 0;
    for cmd in s {
        match cmd.as_str() {
            "login" => auth = true,
            "logout" => auth = false,
            "private" if !auth => ans += 1,
            _ => (),
        }
    }

    println!("{}", ans);
}
