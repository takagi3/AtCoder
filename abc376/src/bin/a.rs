use proconio::input;

fn main() {
    input! {
        n: usize,
        c: u32,
        t: [u32; n],
    }

    println!(
        "{}",
        t.iter()
            .skip(1)
            .fold((1usize, t[0]), |(count, last), &time| {
                if time - last >= c {
                    (count + 1, time)
                } else {
                    (count, last)
                }
            })
            .0
    );
}
