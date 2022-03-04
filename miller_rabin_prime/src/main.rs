//extern crate rand;

use rand::{thread_rng, Rng};
use std::io;

fn modular_expo(mut x: i32, mut y: i32, p: i32) -> i32 {
    let mut result = 1;

    // Update 'x' if it is more than or equal to 'p'
    if x >= p {
        x = x % p;
    }
    while y > 0 {
        // if 'y' is odd then multiply 'x' with result
        if (y & 1) == 1 {
            result = (result * x) % p;
        }
        // 'y' must be even
        y >>= 1; // y = y/2
        x = (x * x) % p;
    }
    result
}

// function is called for all k trials and returns false is n is composite
// returns true if 'n' is 'probably prime' - d is an odd number such that d*2**r = n-1 where r >= 1
fn miller_test(mut d: i32, n: i32) -> bool {
    // Pick a random number in [2..n-2] - corner case, minimum of n > 4
    let random = thread_rng().gen_range(2..=(n - 2));
    let a = 2 + random % (n - 4);

    // compute a^d % n
    let mut x = modular_expo(a, d, n);

    if x == 1 || x == (n - 1) {
        return true;
    }

    /*
    Square 'x' while all of the following is true:
        (i) d does not reach n - 1
        (ii) (x^2) % n is not 1
        (iii) (x^2) % n is not n - 1
    */

    while d != (n - 1) {
        x = (x * x) % n;
        d *= 2;

        if x == 1 {
            return false;
        }
        // this was set to false - leading to incorrect results
        if x == (n - 1) {
            return true;
        }
    }

    // return the composite number
    false
}

fn is_prime(n: i32, k: i32) -> bool {
    // corner cases
    if n <= 1 || n == 4 {
        return false;
    } else if n <= 3 {
        return true;
    }

    // find r such that n = 2^d * r + 1 for some r >= 1
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }

    // iterate given the number of 'k' times
    for _ in 0..k {
        if !miller_test(d, n) {
            return false;
        }
    }

    true
}

fn main() {
    // number of iterations
    let k = 4;
    let mut target = String::new();

    println!("Please input your target number:");
    io::stdin()
        .read_line(&mut target)
        .expect("Failed to read line");

    let mut input = 0;
    match target.trim().parse() {
        Ok(val) => input = val,
        Err(_) => {
            eprintln!("ERROR: Please enter a number.\n")
        }
    };

    // checking if a single number is prime
    println!("Is {} prime? {}\n", input, is_prime(input, k));

    // checking for prime numbers in a range of numbers
    println!("All primes smaller than {}:", input);
    for i in 1..input {
        if is_prime(i, k) {
            print!("{} ", i);
        }
    }
    println!()
}
