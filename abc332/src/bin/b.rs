use proconio::input;

fn main() {
    input! {
        k: usize,
        g: usize,
        m: usize,
    }

    let (mut glass, mut mag) = (0, 0);
    for _ in 0..k {
        if glass == g {
            glass = 0;
        } else if mag == 0 {
            mag = m;
        } else {
            let pour = (g - glass).min(mag);
            glass += pour;
            mag -= pour;
        }
    }

    println!("{} {}", glass, mag);
}