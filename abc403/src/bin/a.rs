use proconio::input;

fn main() {
    input! {
        n : usize,
        a: [u32; n],
    }

    println!("{}", a.iter().step_by(2).sum::<u32>());
}
