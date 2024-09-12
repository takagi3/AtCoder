use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans: u32;
    match s.as_str() {
        "Monday" => { ans = 5 }
        "Tuesday" => { ans = 4 }
        "Wednesday" => { ans = 3 }
        "Thursday" => { ans = 2 }
        "Friday" => { ans = 1 }
        _ => { ans = 0 }
    }

    println!("{}", ans);
}
