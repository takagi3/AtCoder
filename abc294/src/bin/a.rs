use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let ans: Vec<u32> = a.iter()
        .filter_map(|&i| if i % 2 == 0 { Some(i) } else { None })
        .collect();

    println!("{}", ans.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
