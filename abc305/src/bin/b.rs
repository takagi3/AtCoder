use proconio::input;

const DISTANCES: [i32; 7] = [0, 3, 4, 8, 9, 14, 23];

fn char_to_index(c: char) -> usize {
    (c as u8 - b'A') as usize
}

fn main() {
    input! {
        p: char,
        q: char,
    }

    let ans = (DISTANCES[char_to_index(p)] - DISTANCES[char_to_index(q)]).abs();
    println!("{}", ans);
}
