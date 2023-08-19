use num::{traits::PrimInt, zero};

pub(crate) fn reduce<T: PrimInt>(a: T, b: T) -> (T, T) {
    let g = gcd(a, b);
    (a / g, b / g)
}

pub(crate) fn sign_preserving_mod(a: i32, b: i32) -> i32 {
    (a % b + b) % b
}

fn gcd<T: PrimInt>(a: T, b: T) -> T {
    let mut a = a;
    let mut b = b;
    while a % b > zero() {
        let t = a % b;
        a = b;
        b = t;
    }
    b
}

pub(crate) fn greatest_prime_factor<T: PrimInt>(a: T) -> T {
    let mut a = a;
    let mut p = num::cast(2).unwrap();

    while a > num::one() {
        if a % p == num::zero() {
            a = a / p;
        } else {
            p = p + num::one();
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greatest_prime_factor() {
        assert_eq!(greatest_prime_factor(5), 5);
        assert_eq!(greatest_prime_factor(15), 5);
        assert_eq!(greatest_prime_factor(14), 7);
    }
}
