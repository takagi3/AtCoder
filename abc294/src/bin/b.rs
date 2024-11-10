use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u8;w];h],
    }

    for row in a.iter() {
        let line: String = row.iter()
            .map(|&x| if x == 0 { '.' } else { (x + 64) as char })
            .collect();
        println!("{}", line);
    }
}
