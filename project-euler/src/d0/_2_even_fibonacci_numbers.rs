fn _solution() -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut c = 0;
    let mut sum = 0;

    while c < 4000000 {
        c = a + b;
        a = b;
        b = c;

        if c % 2 == 0 {
            sum += c;
        }
    }

    sum
}

#[test]
fn test() {
    println!("Sum: {}", _solution());
}
