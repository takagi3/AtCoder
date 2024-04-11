use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    a.sort();

    println!("{}", a.iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
