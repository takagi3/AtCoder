use proconio::input;

fn main() {
    input! {
        q: usize
    }

    let mut cards = vec![0; 100];
    for _ in 0..q {
        input! { t: i32 }
        match t {
            1 => {
                input! { x: i32 }
                cards.push(x);
            }
            2 => println!("{}", cards.pop().unwrap()),
            _ => unreachable!(),
        }
    }
}
