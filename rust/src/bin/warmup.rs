// Part I: SML & Rust Warmup

//checks if 42 shows up in the list
fn hitchhiker(list: &[i32]) -> bool {
    match list {
        [] => false,
        // The front number is 42 so we found it
        [42, ..] => true,
        // not 42 so keep checking the rest
        [_, rest @ ..] => hitchhiker(rest),
    }
}

// keeps every other number from the list
fn everyother(list: &[i32]) -> Vec<i32> {
    match list {
        // Nothing left to keep
        [] => vec![],
        //only one number left so it gets skipped
        [_] => vec![],
        //Skip the first number and keep the second and recurse on the rest
        [_, second, rest @ ..] => [vec![*second], everyother(rest)].concat(),
    }
}

// Finds two primes that add up to the number and prints them
fn goldbach(n: i32) {
    match (n > 2, n % 2 == 0) {
        //number is not bigger than 2 so it doesn't work
        (false, _) => println!("{} has to be greater than 2", n),
        // number is odd so it doesn't work
        (_, false) => println!("{} has to be an even number", n),
        // Number is good so start looking for primes at 2
        (true, true) => find_primes(n, 2),
    }
}

// Checks each number starting at p until they are both prime
fn find_primes(n: i32, p: i32) {
    match (is_prime(p), is_prime(n - p)) {
        // Both are prime so print the answer
        (true, true) => println!("{} + {} = {}", p, n - p, n),
        // At least one is not prime so try the next number
        _ => find_primes(n, p + 1),
    }
}

// checks if a number is prime
fn is_prime(n: i32) -> bool {
    match n {
        // Anything less than 2 is not prime
        n if n < 2 => false,
        _ => no_divisors(n, 2),
    }
}

// Checks that nothing from d up to the square root of n divides n
fn no_divisors(n: i32, d: i32) -> bool {
    match (d * d > n, n % d == 0) {
        // Went past the square root so there are no divisors
        (true, _) => true,
        // d divides n evenly so it is not prime
        (false, true) => false,
        // d doesn't divide n so try the next one
        (false, false) => no_divisors(n, d + 1),
    }
}

fn main() {
    //examples to test hitchhiker
    println!("Hitchhiker:");
    println!("{}", hitchhiker(&[1, 2, 42, 4]));
    println!("{}", hitchhiker(&[1, 2, 3, 4]));

    // example from the project doc to test others
    println!("Every Other:");
    println!("{:?}", everyother(&[3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43]));

    //examples to test goldbach and its error checking
    println!("Goldbach:");
    goldbach(28);
    goldbach(4);
    goldbach(100);
    goldbach(15);
    goldbach(2);
}
