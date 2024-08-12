fn _solution() -> u32 {
    let mut num_of_divisors = 0;
    let mut n = 1;

    while num_of_divisors <= 500 {
        let triangle_number = (n * (n + 1)) / 2;
        num_of_divisors = _count_divisors(triangle_number);
        if num_of_divisors > 500 {
            return triangle_number;
        }
        n += 1;
    }

    0
}

fn _count_divisors(n: u32) -> u32 {
    let mut count = 0;
    let sqrt_n = (n as f64).sqrt() as u32;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            count += 2; // i and n/i
        }
    }

    // If n is a perfect square, we count sqrt(n) twice
    if sqrt_n * sqrt_n == n {
        count -= 1;
    }

    count
}

#[test]
fn test() {
    println!("{}", _solution());
}