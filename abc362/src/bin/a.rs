use proconio::input;

fn main() {
    input! {
        r: u32,
        g: u32,
        b: u32,
        c: String,
    }

    println!(
        "{}",
        [("Red", r), ("Green", g), ("Blue", b)]
            .iter()
            .filter(|&&(name, _)| name != c)
            .map(|&(_, value)| value)
            .min()
            .unwrap()
    );
}
