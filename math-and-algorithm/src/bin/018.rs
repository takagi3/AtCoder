use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut price: Vec<u32> = vec![0; 5];
    for i in 0..n {
        price[a[i] / 100] += 1
    }
    let ans: u32 = price[1] * price[4] + price[2] * price[3];

    println!("{}", ans);
}
