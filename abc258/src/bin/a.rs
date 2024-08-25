use proconio::input;

fn main() {
    input! {
        k: u32,
    }

    let mut hh: u32 = 21;
    let mut mm: u32 = 0;
    if k >= 60 {
        hh += 1
    }
    mm += k % 60;

    println!("{0:0>2}:{1:0>2}", hh, mm);
}
