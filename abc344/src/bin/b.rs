use proconio::input;

fn main() {
    let mut ans = Vec::new();
    loop {
        input! { a: u32 }
        ans.push(a);
        if a == 0 {
            break;
        }
    }

    for a in ans.iter().rev() {
        println!("{}", a);
    }
}
