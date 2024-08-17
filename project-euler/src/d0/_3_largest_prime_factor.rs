fn _solution(n: i64) -> i64 {
    let mut num = n;
    let mut largest_prime = 0;
    let mut i = 2;

    while i * i <= num {
        while num % i == 0 {
            num /= i;
            largest_prime = i;
        }

        i += 1;
    }

    if num > 1 {
        largest_prime = num;
    }

    largest_prime
}

#[test]
fn test() {
    println!("{}", _solution(600851475143));
}
