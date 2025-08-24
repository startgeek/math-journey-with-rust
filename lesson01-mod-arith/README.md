so lets start learning.
its a good idea if you have this readme file open on one side of screen and main.rs of this lesson on other side of screen. 

Lets dive into definations and what we are dealing with here.

mod: imagine a closed ciculer numbers exactly like a analog clock when clock passes 12 next stop is 1 noth 13 and mod 12 is exactly is a clock

ex: (11 + 3) mod 12 = 2

now take a look at addmod function in our code: what it does it actually adds moduled numbers before adding it reduces the moduled amount so in our clock example it changes 13 to 1

remeber in rust % is REMINDER not MOODULER

so each % m at beginning makes that number a after moduler number (13 becomes 1). 
the end part which we do a % m outside inner paranteses and then + m is to be sure we wont produce a negative number

if you notices mod_sub is exactly is same only we subtract two numbers

same story for mod_mul until end what is that rem_euclid and why we didnt use regular (% m + m) well short answer is to make rust modular safe (non negative) because reminer (%) in rust isnt safe for negative. ((a⋅b)modm)
this function is part of STD of rust 
(https://doc.rust-lang.org/std/?search=rem_euclid)

mod_pow is interesting one:

base pow a exp and the mod of m 

because even with modern computers POW causes overflow we used "modular exponentiation" which is faster and memory safe.
Cryptographic algorithms like RSA, Diffie-Hellman, and DSA all rely on POW and numbers are way bigger than what we see here so they need a way to deal with this without overflow

Exponentiation by Squaring Also called binary exponentiation.

lets look at the problem in binary level so assume this is our input mod_pow(3, 13, 17)

what is 13 in binary ? ----> 13 = 1101 
basically 2 posion is empty and 1 4 and 8 positions are valid 
so it becomes 313= 3(pow 8)⋅3(pow 4)⋅3(pow 1) as you can see we skipped 2 position.

now remember >> is shift right to left so we deal with 1 then 0 then 1 then 1 .
 .
 .
 ...
 .
 .
 .
 .
 .
 .
 .
 .
 .

 mod_inv:

 a = 3, m = 11
We want to find x such that:


3*x≡1 mod11
so in this scenario a and x should be co-primes menaing they cant have shared factors except 1

so how it get implemented in computer ?
using Extended Euclidean Algorithm