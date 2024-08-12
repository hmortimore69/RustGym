fn _sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut primes = vec![true; n + 1];
    let mut p = 2;

    while p * p <= n {
        if primes[p] {
            for i in (p * p..=n).step_by(p) {
                primes[i] = false;
            }
        }
        p += 1;
    }

    let mut prime_numbers = Vec::new();
    for i in 2..=n {
        if primes[i] {
            prime_numbers.push(i);
        }
    }

    prime_numbers
}

#[test]
fn test() {
    let prime_numbers = _sieve_of_eratosthenes(2_000_000);
    let sum: u64 = prime_numbers.iter().map(|&x| x as u64).sum();
    println!("{}", sum);
}