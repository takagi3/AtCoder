use proconio::input;
type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        a: Mint,
        b: Mint,
        c: Mint,
        d: Mint,
        e: Mint,
        f: Mint,
    }

    let ans: Mint = a * b * c - d * e * f;

    println!("{}", ans);
}
