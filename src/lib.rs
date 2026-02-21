/// Computes the nth Fibonacci number.
///
/// Uses an iterative approach. `fibonacci(0)` returns 0 and `fibonacci(1)` returns 1.
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 2..=n {
                let tmp = a + b;
                a = b;
                b = tmp;
            }
            b
        }
    }
}

/// Returns `true` if `n` is a prime number, `false` otherwise.
///
/// Handles edge cases: 0 and 1 are not prime.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Computes the greatest common divisor of `a` and `b` using the Euclidean algorithm.
///
/// Returns `a` when `b` is 0, and vice versa.
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fibonacci tests
    #[test]
    fn fibonacci_zero() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn fibonacci_one() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn fibonacci_two() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn fibonacci_ten() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn fibonacci_twenty() {
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn fibonacci_thirty() {
        assert_eq!(fibonacci(30), 832040);
    }

    // is_prime tests
    #[test]
    fn is_prime_zero() {
        assert!(!is_prime(0));
    }

    #[test]
    fn is_prime_one() {
        assert!(!is_prime(1));
    }

    #[test]
    fn is_prime_two() {
        assert!(is_prime(2));
    }

    #[test]
    fn is_prime_three() {
        assert!(is_prime(3));
    }

    #[test]
    fn is_prime_small_primes() {
        for p in [5, 7, 11, 13, 17, 19, 23, 29, 31] {
            assert!(is_prime(p), "{p} should be prime");
        }
    }

    #[test]
    fn is_prime_small_composites() {
        for c in [4, 6, 8, 9, 10, 12, 15, 21, 25] {
            assert!(!is_prime(c), "{c} should not be prime");
        }
    }

    #[test]
    fn is_prime_even_numbers() {
        assert!(!is_prime(100));
        assert!(!is_prime(1000));
    }

    #[test]
    fn is_prime_larger_prime() {
        assert!(is_prime(7919));
    }

    #[test]
    fn is_prime_larger_composite() {
        assert!(!is_prime(7917)); // 3 * 2639
    }

    // gcd tests
    #[test]
    fn gcd_both_zero() {
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn gcd_one_zero() {
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn gcd_coprime() {
        assert_eq!(gcd(7, 13), 1);
        assert_eq!(gcd(17, 31), 1);
    }

    #[test]
    fn gcd_identical() {
        assert_eq!(gcd(42, 42), 42);
    }

    #[test]
    fn gcd_known_pairs() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(100, 75), 25);
    }

    #[test]
    fn gcd_commutative() {
        assert_eq!(gcd(12, 8), gcd(8, 12));
        assert_eq!(gcd(54, 24), gcd(24, 54));
    }
}
