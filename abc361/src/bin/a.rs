use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: u32,
        a: [u32; n],
    }

    let mut ans: Vec<u32> = a[..k].iter().copied().collect();
    ans.push(x);
    ans.extend(&a[k..]);

    println!(
        "{}",
        ans.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
