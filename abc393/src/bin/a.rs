use proconio::input;

fn main() {
    input! {
        (s1, s2): (String, String),
    }

    println!(
        "{}",
        match (s1.as_str(), s2.as_str()) {
            ("sick", "sick") => 1,
            ("sick", "fine") => 2,
            ("fine", "sick") => 3,
            ("fine", "fine") => 4,
            _ => 0,
        }
    );
}
