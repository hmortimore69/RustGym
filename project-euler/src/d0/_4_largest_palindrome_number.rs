fn _solution() -> i32 {
    let mut m = 999;
    let mut largest_palindrome = 0;

    while m > 99 {
        let mut n = 999;

        while n > 99 {
            let product = m * n;
            if product < largest_palindrome {
                break;
            } else if _is_palindrome(product) {
                largest_palindrome = product;
            }
            n -= 1;
        }
        m -= 1;
    }

    largest_palindrome
}

fn _is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

#[test]
fn test() {
    println!("Largest palindrome: {}", _solution());
}
