fn is_prime(n: i32) -> bool {
    // 1 is not prime as it is divisible only by itself
    if n <= 1 {
        return false;
    }
    // 2 is prime, despite being even, 3 is prime
    else if n == 2 || n == 3 {
        return true;
    }
    // if n is divisible by 2 or 3 it is a composite number (not prime)
    else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    // base cases are complete and now further testing required
    // establish a variable from 5 to increment with each attempt
    let mut i = 5.0;

    // while loop to cast n as f64 (sqrt() requires f64 due to it likely returning a decimal)
    // sqrt() is used
    while i <= (n as f64).sqrt() {
        // if n is divisible by the incrementing value e.g. n=49 & i=7, it is not prime
        if n as f64 % i == 0.0 {
            return false;
        }
        // if condition not met - increment by 1 to get next value
        i += 1.0;
    }
    // if while condition is broken - your number is prime
    true
}

fn main() {
    println!("{}", is_prime(2));
    println!("{}", is_prime(37));
    println!("{}", is_prime(81));
    println!("{}", is_prime(97));
    println!("{}", is_prime(5));
}
