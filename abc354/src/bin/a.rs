use proconio::input;

fn main() {
    input! {
        h: i32,
    }

    let mut _t = 0;
    for i in 0.. {
        _t = (1 << (i + 1)) - 1;
        if _t > h {
            println!("{}", i + 1);
            break;
        }
    }
}