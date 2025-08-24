fn mod_add(a: i64, b: i64, m: i64) -> i64 {
    ((a % m + b % m) % m + m) % m
}

fn mod_sub(a: i64, b: i64, m: i64) -> i64 {
    ((a % m - b % m) % m + m) % m
}

fn mod_mul(a: i64, b: i64, m: i64) -> i64 {
    ((a % m) * (b % m)).rem_euclid(m)
}

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1;
    base = base % m;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % m;
        }
        exp >>= 1;
        base = (base * base) % m;
    }
    result
}

fn mod_inv(a: i64, m: i64) -> Option<i64> {
    let (gcd, x, _) = extended_gcd(a, m);
    if gcd != 1 {
        None
    } else {
        Some((x % m + m) % m)
    }
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a / b) * y1)
    }
}

fn main() {
    let a = 42;
    let b = 17;
    let m = 101;

    println!("mod_add({}, {}) mod {} = {}", a, b, m, mod_add(a, b, m));
    println!("mod_sub({}, {}) mod {} = {}", a, b, m, mod_sub(a, b, m));
    println!("mod_mul({}, {}) mod {} = {}", a, b, m, mod_mul(a, b, m));
    println!("mod_pow({}, {}) mod {} = {}", a, b, m, mod_pow(a, b, m));

    match mod_inv(a, m) {
        Some(inv) => println!("mod_inv({}, {}) = {}", a, m, inv),
        None => println!("mod_inv({}, {}) does not exist", a, m),
    }
}
