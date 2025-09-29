use proconio::input;

fn main() {
    input! {
        a: [usize; 7],
    }

    let mut cards: Vec<u32> = vec![0; 14];
    a.iter().for_each(|&x| cards[x] += 1);

    println!(
        "{}",
        if cards.iter().filter(|&x| *x >= 3).count() >= 1
            && cards.iter().filter(|&x| *x >= 2).count() >= 2
        {
            "Yes"
        } else {
            "No"
        }
    );
}
