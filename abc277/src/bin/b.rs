use std::collections::HashSet;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut ans: &str = "Yes";
    let suit: Vec<char> = vec!['H', 'D', 'C', 'S'];
    let number: Vec<char> = vec!['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];
    let mut card: HashSet<Vec<char>> = HashSet::new();
    for i in 0..n {
        if !suit.contains(&s[i][0]) || !number.contains(&s[i][1]) || card.contains(&s[i]) {
            ans = "No";
            break;
        }
        card.insert(s[i].clone());
    }

    println!("{}", ans);
}
