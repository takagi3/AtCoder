use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
        q: usize,
    }

    for _ in 0..q {
        input! {
            mode: u32,
        }
        match mode {
            1 => {
                input! {
                    k: Usize1,
                    x: u32,
                }
                a[k] = x
            }
            2 => {
                input! {
                    k: Usize1,
                }
                println!("{}", a[k]);
            }
            _ => continue
        }
    }
}
