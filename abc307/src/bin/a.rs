use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n * 7],
    }

    let ans = a
        .chunks(7)
        .map(|week| week.iter().sum::<u32>())
        .collect::<Vec<_>>();

    println!(
        "{}",
        ans.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
