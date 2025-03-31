use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [usize; q],
    }

    let mut tooth: Vec<bool> = vec![true; n];
    for x in t {
        tooth[x - 1] ^= true;
    }

    println!("{}", tooth.iter().copied().filter(|b| *b).count());
}
