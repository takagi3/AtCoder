use proconio::input;

fn main() {
    input! {
        sab: char,
        sac: char,
        sbc: char,
    }

    let ans = if sab != sac {
        "A"
    } else if sab == sbc {
        "B"
    } else {
        "C"
    };

    println!("{}", ans);
}
