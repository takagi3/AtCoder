use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut first_b_index = None;
    let mut alternating_b_positions = false;
    let mut found_k_between_r = false;
    let mut r_count = 0;
    for (i, &c) in s.iter().enumerate() {
        match c {
            'B' => {
                if let Some(b_index) = first_b_index {
                    if b_index % 2 != i % 2 {
                        alternating_b_positions = true;
                    }
                } else {
                    first_b_index = Some(i);
                }
            }
            'R' => {
                r_count += 1;
            }
            'K' => {
                if r_count == 1 {
                    found_k_between_r = true;
                }
            }
            _ => {}
        }
    }
    let ans = if alternating_b_positions && found_k_between_r {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
