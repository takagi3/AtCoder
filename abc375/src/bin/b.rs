use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(f64, f64); n],
    }

    let mut path = Vec::with_capacity(n + 2);
    path.push((0.0, 0.0));
    path.extend(points);
    path.push((0.0, 0.0));

    println!(
        "{:.12}",
        path.windows(2)
            .map(|w| {
                let (x1, y1) = w[0];
                let (x2, y2) = w[1];
                ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
            })
            .sum::<f64>()
    );
}
