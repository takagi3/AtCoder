use proconio::input;

fn main() {
    input! {
        p: [u8; 26],
    }

    let mut ans: Vec<char> = vec![];
    for i in 0..26 {
        ans.push((p[i] + 96) as char)
    }

    println!("{}", ans.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("")
    );
}
