use proconio::input;

fn main() {
    input! { mut m: i32 }
    
    const POWERS: [(i32, usize); 11] = [
        (3_i32.pow(10), 10), (3_i32.pow(9), 9),  (3_i32.pow(8), 8),
        (3_i32.pow(7), 7),  (3_i32.pow(6), 6),  (3_i32.pow(5), 5),
        (3_i32.pow(4), 4),  (3_i32.pow(3), 3),  (3_i32.pow(2), 2),
        (3_i32.pow(1), 1),  (3_i32.pow(0), 0),
    ];

    let mut result = Vec::new();
    for &(value, exp) in &POWERS {
        while m >= value {
            m -= value;
            result.push(exp);
        }
        if m == 0 {
            break;
        }
    }

    println!("{}", result.len());
    println!("{}", result.iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join(" ")
    );
}