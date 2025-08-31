What is Number Theory?

Number theory is the branch of pure mathematics that studies integers (whole numbers like -2, -1, 0, 1, 2, …) and their properties.

It’s often called the “queen of mathematics” because it touches deep truths hidden inside simple questions like:

Is a number divisible by another?

What makes a number prime?

Can every number be written in a special form?

How do numbers behave when we apply modular arithmetic (remainders)?


Core Building Blocks
1. Divisibility

If a divides b (written a | b), then b can be written as a × k for some integer k.

Example: 3 | 12 because 12 = 3 × 4.

2. Prime Numbers

A prime is a number greater than 1 that has no divisors except 1 and itself.

Examples: 2, 3, 5, 7, 11…

Fundamental Theorem of Arithmetic: Every integer greater than 1 can be written uniquely as a product of primes.

Example: 60 = 2² × 3 × 5.

3. Greatest Common Divisor (GCD)

GCD of two numbers is the largest integer that divides both.

Example: gcd(18, 24) = 6.

Can be computed using the Euclidean Algorithm (subtract or divide repeatedly).

4. Modular Arithmetic (Clock Math)

Instead of looking at numbers directly, we look at their remainders.

Example: 17 mod 5 = 2 because when dividing 17 by 5, remainder is 2.

This idea is used in cryptography (RSA, elliptic curves, etc.), which I know you’re diving into.

5. Congruences

If a and b have the same remainder when divided by n, we say:
a ≡ b (mod n)

Example: 23 ≡ 5 (mod 9) because both leave remainder 5.

6. Diophantine Equations

Equations where solutions must be integers.

Example: 2x + 3y = 7. The solutions must be integers, not fractions.

7. Special Topics

Fermat’s Little Theorem: If p is prime, then a^(p-1) ≡ 1 (mod p) for any a not divisible by p.

Chinese Remainder Theorem: Lets you solve simultaneous modular equations.

Quadratic residues: Whether a number has a square root modulo n.