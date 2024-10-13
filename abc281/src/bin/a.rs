use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut ans: Vec<u32> = vec![];
    for i in (0..=n).rev() {
        ans.push(i)
    }

    println!("{}", ans.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
    );
}
