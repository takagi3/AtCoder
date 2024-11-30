use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let digits = n.to_string().len();
    let ans = if digits > 3 {
        let round_base = 10_i64.pow((digits - 3) as u32);
        (n / round_base) * round_base
    } else {
        n
    };

    println!("{}", ans);
}
