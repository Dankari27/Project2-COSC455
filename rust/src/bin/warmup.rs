// Part I: SML & Rust Warmup
#![allow(dead_code, unused_variables)] // Hides warnings while things are still unfinished

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
    todo!()
}

fn main() {
    //examples to test hitchhiker
    println!("Hitchhiker:");
    println!("{}", hitchhiker(&[1, 2, 42, 4]));
    println!("{}", hitchhiker(&[1, 2, 3, 4]));

    // example from the project doc to test others
    println!("Every Other:");
    println!("{:?}", everyother(&[3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43]));

    // Do goldbach here later
}
