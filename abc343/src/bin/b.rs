use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
    }

    for row in &a {
        for (j, &val) in row.iter().enumerate() {
            if val == 1 {
                print!("{} ", j + 1);
            }
        }
        println!();
    }
}