pub fn reduce(a: i32, b: i32) -> (i32, i32) {
    let g = gcd(a, b);
    (a / g, b / g)
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
