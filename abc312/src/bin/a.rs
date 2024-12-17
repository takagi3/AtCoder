use proconio::input;

fn main() {
    input! {
        s: String,
    }

    const CHORDS: [&str; 7] = ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];
    let ans = if CHORDS.iter().any(|&chord| chord == s) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
