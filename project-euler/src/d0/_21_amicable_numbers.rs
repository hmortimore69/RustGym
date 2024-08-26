fn _solution() -> u32 {
    let mut sum = 0;
    for a in 2..10000 {
        let b = d(a);
        if a != b && a == d(b) {
            sum += a;
        }
    }

    sum
}

fn d(n: u32) -> u32 {
    (1..n).filter(|&i| n % i == 0).sum()
}

#[test]
fn test() {
    println!("{}", _solution());
}
