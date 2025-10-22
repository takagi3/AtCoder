use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let chars: Vec<_> = s.chars().collect();

    if let Some(c) = ('a'..='z').find(|&c| !chars.contains(&c)) {
        println!("{}", c);
    }
}
