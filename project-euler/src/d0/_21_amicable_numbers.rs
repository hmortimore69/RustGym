fn _solution() -> u32 {
    let mut sum = 0;
    for a in 2..10000 {
        let b = _d(a);
        if a != b && a == _d(b) {
            sum += a;
        }
    }

    sum
}

fn _d(n: u32) -> u32 {
    (1..n).filter(|&i| n % i == 0).sum()
}

#[test]
fn test() {
    println!("{}", _solution());
}
