use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [String; n],
        b: [String; n],
    }

    for (i, (a_line, b_line)) in a.iter().zip(b.iter()).enumerate() {
        if let Some(j) = a_line
            .chars()
            .zip(b_line.chars())
            .position(|(ac, bc)| ac != bc)
        {
            println!("{} {}", i + 1, j + 1);
        }
    }
}
