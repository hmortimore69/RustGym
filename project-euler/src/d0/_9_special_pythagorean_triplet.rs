fn _solution() -> u64 {
    for a in 1..1000 {
        for b in a + 1..1000 {
            if a + b >= 1000 {
                break;
            }

            let c = 1000 - a - b;

            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    
    0
}

#[test]
fn test() {
    println!("{}", _solution());
}
