use proconio::input;

fn main() {
    input! {
        k: u8,
    }

    let mut ans: Vec<char> = vec![];
    for i in 0..k {
        ans.push((i + 65) as char)
    }

    println!("{}", ans.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("")
    );
}
