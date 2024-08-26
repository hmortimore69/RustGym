fn _sieve_of_eratosthenes(n: usize) {
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
    let mut count = 0;

    for i in 2..=n {
        if primes[i] {
            count += 1;
            if count == 10001 {
                println!("{}", i);
                break;
            }
        }
    }
}

#[test]
fn test() {
    _sieve_of_eratosthenes(200_000);
}
