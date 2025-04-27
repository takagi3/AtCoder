use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i64,
        t: [i64; n],
    }

    t.into_iter()
        .scan(0_i64, |now, task| {
            *now = (*now).max(task) + a;
            Some(*now)
        })
        .for_each(|time| println!("{}", time));
}
