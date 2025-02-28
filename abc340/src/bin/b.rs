use proconio::input;

fn main() {
    input! {
        n: usize,
        queries: [(usize, usize); n],
    }

    let mut stack = Vec::with_capacity(n);
    for (kind, val) in queries {
        match kind {
            1 => {
                stack.push(val);
            }
            2 => {
                println!("{}", stack[stack.len() - val]);
            }
            _ => (),
        }
    }
}
