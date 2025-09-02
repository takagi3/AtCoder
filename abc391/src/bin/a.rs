use proconio::input;

fn main() {
    input! {
        d: String,
    }

    println!(
        "{}",
        match d.as_str() {
            "N" => "S",
            "NE" => "SW",
            "E" => "W",
            "SE" => "NW",
            "S" => "N",
            "SW" => "NE",
            "W" => "E",
            "NW" => "SE",
            _ => "",
        }
    );
}
