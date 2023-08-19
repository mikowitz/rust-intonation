pub(crate) fn reduce(a: i32, b: i32) -> (i32, i32) {
    let g = gcd(a, b);
    (a / g, b / g)
}

pub(crate) fn sign_preserving_mod(a: i32, b: i32) -> i32 {
    (a % b + b) % b
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while a % b > 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    b
}

pub(crate) fn greatest_prime_factor(a: i32) -> i32 {
    let mut a = a;
    let mut p = 2;

    while a > 1 {
        if a % p == 0 {
            a /= p;
        } else {
            p += 1;
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
