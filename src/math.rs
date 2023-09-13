use num::{traits::PrimInt, zero};

pub(crate) fn normalize_pair<T: PrimInt>(a: T, b: T) -> (T, T) {
    let f: f32 = num::cast(a / b).unwrap();
    let two: T = num::cast(2i32).unwrap();

    match f {
        f if f < 1. => normalize_pair(a * two, b),
        f if f >= 2. => normalize_pair(a, b * two),
        _ => (a, b),
    }
}
// let f: f64 = self.into();
// let two: T = num::cast(2i32).unwrap();
//
// match f {
//     f if f < 1. => Self::new(self.numer * two, self.denom),
//     f if f >= 2. => Self::new(self.numer, self.denom * two),
//     _ => Self::new(self.numer, self.denom),
// }

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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_greatest_prime_factor() {
        assert_eq!(greatest_prime_factor(5), 5);
        assert_eq!(greatest_prime_factor(15), 5);
        assert_eq!(greatest_prime_factor(14), 7);
    }
}
