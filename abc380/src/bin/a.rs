use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    }

    let counts = n.iter().fold([0_u32; 3], |mut acc, &c| {
        match c {
            '1' => acc[0] += 1,
            '2' => acc[1] += 1,
            '3' => acc[2] += 1,
            _ => {}
        }
        acc
    });

    println!("{}", if counts == [1, 2, 3] { "Yes" } else { "No" });
}
