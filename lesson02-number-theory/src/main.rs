use rand::Rng;

// --------- Core arithmetic ---------
// What it does: Computes the greatest common divisor using the Euclidean algorithm.
// Why it works: If a = bq + r, then gcd(a, b) = gcd(b, r). Repeating this until remainder is 0 gives the GCD.
// Why while b != 0: Loop until the remainder chain ends.
// divisions—very fast.
// Crypto relevance: Used everywhere—checking coprimality (e.g., gcd(e, φ(n)) = 1 in RSA), simplifying CRT preconditions, etc.
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
// What it does: Returns g = gcd(a, b) and integers x, y such that ax+by=g.
// for any two integers a and b, there exist integers x and y such that ax + by equals their greatest common divisor. It can also refer to Bézout's theorem in algebraic
// Base case: If b == 0, then gcd(a,0) = a and the Bézout identity is a*1 + 0*0 = a, so (1,0).
// Back‑substitution: As recursion unwinds, we compute coefficients for the previous pair.
// Why signed (i128) not unsigned: Coefficients x, y can be negative
// Crypto relevance: This is the engine for computing modular inverses (critical for RSA, ECC).

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = extended_gcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

// What it does: Computes the modular inverse of a mod m, i.e., the x such that a⋅x≡1(modm).
// Normalization: a.rem_euclid(m) guarantees a non‑negative representative of a mod m.
// Existence: Inverse exists iff gcd(a, m) = 1. Otherwise None.
// Returning positive rep: (x mod m) makes the inverse fall in [0, m-1].

// Crypto relevance:
// RSA: d = e^{-1} mod φ(n)
// ECC: inversion in the field 𝔽_p for slope computations
// Edge cases: If m ≤ 0, or a not coprime to m, you get None.

fn mod_inv(a: i128, m: i128) -> Option<i128> {
    // Normalize a into [0, m-1] before EEA
    let a_norm = a.rem_euclid(m);
    let (g, x, _) = extended_gcd(a_norm, m);
    if g != 1 { None } else { Some(x.rem_euclid(m)) }
}
// What it does: Fast modular exponentiation (binary exponentiation). we already good at this from previous lesson
fn mod_pow(mut base: u128, mut exp: u128, m: u128) -> u128 {
    if m == 1 { return 0; }
    base %= m;
    let mut res: u128 = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res * base) % m;
        }
        base = (base * base) % m;
        exp >>= 1;
    }
    res
}

// --------- Euler's totient (phi) via trial factoring ---------
// For learning/demo on small n.
// What it does: Computes Euler’s totient (count of integers in [1..n] coprime to n) by trial factoring.
// Euler's totient function, or the phi function (φ(n)), 
// counts the positive integers up to a given integer n that are relatively prime (or coprime) to n. 
// remeber Two numbers are relatively prime if their only common positive factor is 1.
fn phi(mut n: u128) -> u128 {
    if n == 0 { return 0; }
    let mut result = n;
    let mut p: u128 = 2;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 { n /= p; }
            result = result / p * (p - 1);
        }
        p += 1;
    }
    if n > 1 {
        result = result / n * (n - 1);
    }
    result
}

// --------- Chinese Remainder Theorem (two congruences) ---------
// Solve x ≡ a (mod m), x ≡ b (mod n) for coprime m, n.
// Returns (x, lcm) with lcm = m*n.

// The Chinese Remainder Theorem (CRT) states that if you know the remainders of an integer when divided by several pairwise coprime integers (numbers with no common factors), 
// you can uniquely determine the remainder when the integer is divided by the product of those integers.
fn crt_pair(a: i128, m: i128, b: i128, n: i128) -> Option<(i128, i128)> {
    if m <= 0 || n <= 0 { return None; }
    if gcd(m as u128, n as u128) != 1 {
        return None;
    }
    // a + m*t ≡ b (mod n) -> m*t ≡ (b - a) (mod n)
    let rhs = (b - a).rem_euclid(n);
    let inv_m_mod_n = mod_inv(m.rem_euclid(n), n)?;
    let t = (rhs * inv_m_mod_n).rem_euclid(n);
    let lcm = m.checked_mul(n)?;
    let x = (a + m * t).rem_euclid(lcm);
    Some((x, lcm))
}

// --------- Miller–Rabin (deterministic for 64-bit) ---------
//is this number prime?
const MR_BASES_U64: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];

fn is_probable_prime_u64(n: u64) -> bool {
    if n < 2 { return false; }
    // quick small-prime check
    for p in [2u64,3,5,7,11,13,17,19,23,29,31,37] {
        if n == p { return true; }
        if n % p == 0 && n != p { return false; }
    }
    // write n-1 = d * 2^s with d odd
    let mut d = (n - 1) as u128;
    let mut s = 0;
    while d % 2 == 0 { d /= 2; s += 1; }

    'outer: for &a in MR_BASES_U64.iter() {
        if a >= n { continue; }
        let mut x = mod_pow(a as u128, d, n as u128);
        if x == 1 || x == (n - 1) as u128 { continue; }
        for _ in 1..s {
            x = (x * x) % n as u128;
            if x == (n - 1) as u128 { continue 'outer; }
        }
        return false;
    }
    true
}
//obvious
fn generate_prime_u64(range: std::ops::Range<u64>) -> u64 {
    let mut rng = rand::thread_rng();
    loop {
        let mut k = rng.gen_range(range.clone()) | 1; // odd candidate
        if k < 3 { k = 3; }
        if is_probable_prime_u64(k) { return k; }
    }
}

// --------- Demo helpers ---------
// ok finally we can use all the building blocks that we learned until now
//
fn demo_fermat_euler(a: u128, n: u128) {
    if n == 0 { return; }
    println!("demo of fermat_euler a = {a}, n = {n}");
    // Fermat (only when n fits u64 and is prime)
    if let Ok(n_u64) = u64::try_from(n) {
        if is_probable_prime_u64(n_u64) {
            let v = mod_pow(a % n, (n - 1) as u128, n);
            println!("Fermat: a^(n-1) mod n = {v} (prime n)");
        }
    }
    // Euler (always valid for gcd(a, n) = 1)
    let phi_n = phi(n);
    let v2 = mod_pow(a % n, phi_n, n);
    println!("Euler: a^phi(n) mod n = {v2}, where phi(n) = {phi_n}");
}
//Chinese Remainder Theorem

fn demo_crt(a: i128, m: i128, b: i128, n: i128) {
    match crt_pair(a, m, b, n) {
        Some((x, lcm)) => {
            println!("CRT solution: x ≡ {x} (mod {lcm})");
            println!("  Check: x mod {m} = {}", x.rem_euclid(m));
            println!("         x mod {n} = {}", x.rem_euclid(n));
        }
        None => println!("CRT failed: moduli {m} and {n} must be positive and coprime."),
    }
}

fn main() {
    println!("== Lesson 2: Number Theory Toolkit ==");

    // 1) Fermat/Euler demos
    demo_fermat_euler(7, 17);  // prime modulus (Fermat applies)
    demo_fermat_euler(7, 40);  // composite modulus (Euler applies)

    // 2) CRT demo: x ≡ 2 (mod 3), x ≡ 3 (mod 5) -> x ≡ 8 (mod 15)
    demo_crt(2, 3, 3, 5);

    // 3) Modular inverse demo
    let a = 42i128;
    let m = 101i128;
    let inv = mod_inv(a, m).unwrap();
    println!("mod_inv({a}, {m}) = {inv}  // check: {}",
             (a * inv).rem_euclid(m));

    // 4) Miller–Rabin & prime generation
    let candidate = 1_000_000_007u64; // known prime
    println!("is_probable_prime({candidate}) = {}", is_probable_prime_u64(candidate));
    let rand_prime = generate_prime_u64(100_000..200_000);
    println!("random prime in [100k,200k): {rand_prime}");
}
