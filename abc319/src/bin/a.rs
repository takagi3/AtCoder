use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    }

    let player: HashMap<&str, u32> = HashMap::from([
        ("tourist", 3858),
        ("ksun48", 3679),
        ("Benq", 3658),
        ("Um_nik", 3648),
        ("apiad", 3638),
        ("Stonefeang", 3630),
        ("ecnerwala", 3613),
        ("mnbvmar", 3555),
        ("newbiedmy", 3516),
        ("semiexp", 3481),
    ]);

    if let Some(rating) = player.get(s.as_str()) {
        println!("{}", rating);
    }
}
