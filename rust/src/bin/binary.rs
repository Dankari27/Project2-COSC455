// Part II: Binary Addition/Subtraction

// Adds two lists of 1s and 0s
fn binary_addition(a: &[i32], b: &[i32]) -> Vec<i32> {
    //reverse both lists and then turn them into booleans
    let a_bits = reverse(&to_bools(a));
    let b_bits = reverse(&to_bools(b));
    // add them
    to_ints(&reverse(&do_binary_addition(&a_bits, &b_bits, false)))
}

// Does the main adding work bit by bit
fn do_binary_addition(a: &[bool], b: &[bool], carry: bool) -> Vec<bool> {
    match (a, b) {
        // both lists ran out so only the carry might be left over
        ([], []) => finish_binary_add(&[], carry),
        // One list ran out so pass the other list and the carry along the last
        ([], rest) | (rest, []) => finish_binary_add(rest, carry),
        // Both lists still have bits so add the two front bits along with the carry
        ([x, xs @ ..], [y, ys @ ..]) => {
            // Pattern match on the three bits to figure out what the sum and new carry are
            let (sum, new_carry) = match (*x, *y, carry) {
                (false, false, false) => (false, false), // 0 + 0 + 0 = 0
                (true, false, false)
                | (false, true, false)
                | (false, false, true) => (true, false), // only one bit is set so 1
                (true, true, false)
                | (true, false, true)
                | (false, true, true) => (false, true), // two bits are set so 10
                (true, true, true) => (true, true),    // all three bits are set so 11
            };
            // Put the sum bit in front and recurse on the rest of both lists
            [vec![sum], do_binary_addition(xs, ys, new_carry)].concat()
        }
    }
}

// finishes up when one list runs out before the other
fn finish_binary_add(rest: &[bool], carry: bool) -> Vec<bool> {
    match (rest, carry) {
        // Nothing left and no carry finish
        ([], false) => vec![],
        //Nothing left but there is still a carry
        ([], true) => vec![true],
        // no carry anymore, so the rest of the bits are copied
        (bits, false) => bits.to_vec(),
        // A 0 plus the carry becomes 1 and then stops
        ([false, tail @ ..], true) => [vec![true], tail.to_vec()].concat(),
        // A 1 plus the carry becomes 0 and the carry continues
        ([true, tail @ ..], true) => [vec![false], finish_binary_add(tail, true)].concat(),
    }
}

//Subtracts the second list from the first
fn binary_subtraction(a: &[i32], b: &[i32]) -> Vec<i32> {
    // Reverse both lists and turn them into bools
    let a_bits = reverse(&to_bools(a));
    let b_bits = reverse(&to_bools(b));
    // Pad both lists with 0s so they are the same length
    let a_padded = pad_bits(&a_bits, &b_bits);
    let b_padded = pad_bits(&b_bits, &a_bits);
    // Flip every bit of the second list and get the compliment
    let sum = do_binary_addition(&a_padded, &flip_bits(&b_padded), true);
    // Drop the extra carry bit at the end
    remove_leading_zeros(&to_ints(&reverse(&keep_same_length(&sum, &a_padded))))
}

// turns a list of 1s and 0s into a list of bools
fn to_bools(bits: &[i32]) -> Vec<bool> {
    match bits {
        // nothing left to convert
        [] => vec![],
        // A 0 becomes false
        [0, rest @ ..] => [vec![false], to_bools(rest)].concat(),
        // Anything else is assumed to be a 1 and becomes true
        [_, rest @ ..] => [vec![true], to_bools(rest)].concat(),
    }
}

// Turns a list of bools back into a list of 1s and 0s
fn to_ints(bits: &[bool]) -> Vec<i32> {
    match bits {
        //Nothing left to convert
        [] => vec![],
        //false becomes a 0
        [false, rest @ ..] => [vec![0], to_ints(rest)].concat(),
        // true becomes a 1
        [true, rest @ ..] => [vec![1], to_ints(rest)].concat(),
    }
}

// Reverses a list by moving the front item to the back
fn reverse<T: Clone>(list: &[T]) -> Vec<T> {
    match list {
        // An empty list is already reversed
        [] => vec![],
        // reverse the rest and put the first item at the end
        [first, rest @ ..] => [reverse(rest), vec![first.clone()]].concat(),
    }
}

//flips every bit so 0s become 1s and 1s become 0s
fn flip_bits(bits: &[bool]) -> Vec<bool> {
    match bits {
        // Nothing left to flip
        [] => vec![],
        // Flip the front bit and recurse on the rest
        [first, rest @ ..] => [vec![!*first], flip_bits(rest)].concat(),
    }
}

// Adds 0s to the end of the first list until it is as long as the second list
fn pad_bits(bits: &[bool], other: &[bool]) -> Vec<bool> {
    match (bits, other) {
        // The other list ran out so the rest of the bits are copied
        (rest, []) => rest.to_vec(),
        ([], [_, os @ ..]) => [vec![false], pad_bits(&[], os)].concat(),
        // Both lists still have bits so keep the front bit
        ([x, xs @ ..], [_, os @ ..]) => [vec![*x], pad_bits(xs, os)].concat(),
    }
}

// keeps only as many bits as the second list has
fn keep_same_length(bits: &[bool], other: &[bool]) -> Vec<bool> {
    match (bits, other) {
        // Either list ran out so stop here
        ([], _) | (_, []) => vec![],
        // If both lists still have bits left
        ([x, xs @ ..], [_, os @ ..]) => [vec![*x], keep_same_length(xs, os)].concat(),
    }
}

//gets rid of extra 0s at the front but always leaves at least one bit
fn remove_leading_zeros(bits: &[i32]) -> Vec<i32> {
    match bits {
        //A 0 with more bits after it can be dropped
        [0, rest @ ..] if !rest.is_empty() => remove_leading_zeros(rest),
        _ => bits.to_vec(),
    }
}

// Turns a list of 1s and 0s into a string so it is easier to read
fn bits_to_string(bits: &[i32]) -> String {
    match bits {
        [] => String::new(),
        // Put the front bit in the string
        [first, rest @ ..] => first.to_string() + &bits_to_string(rest),
    }
}

//Prints out an addition problem and its answer
fn show_addition(a: &[i32], b: &[i32]) {
    println!(
        "{} + {} = {}",
        bits_to_string(a),
        bits_to_string(b),
        bits_to_string(&binary_addition(a, b))
    );
}

//Prints out a subtraction problem and its answer
fn show_subtraction(a: &[i32], b: &[i32]) {
    println!(
        "{} - {} = {}",
        bits_to_string(a),
        bits_to_string(b),
        bits_to_string(&binary_subtraction(a, b))
    );
}

fn main() {
    // Examples from the project doc to test
    println!("Binary Addition:");
    show_addition(&[1, 1, 1, 1, 0], &[1, 0, 1, 1]);
    show_addition(&[1, 0, 0, 1, 1, 0, 1], &[0, 0, 1, 0, 0, 1, 0]);
    show_addition(&[1, 0, 0, 1, 0, 0, 1], &[0, 0, 1, 1, 0, 0, 1]);
    show_addition(&[1, 0, 0, 0, 1, 1, 1], &[1, 0, 1, 1, 0]);


    println!("Binary Subtraction:");
    show_subtraction(&[1, 0, 1, 0, 0, 1], &[1, 0, 1, 1]);
    show_subtraction(&[1, 0, 1, 1, 1, 1, 1], &[0, 0, 1, 0, 0, 1, 0]);
    show_subtraction(&[1, 1, 0, 0, 0, 1, 0], &[0, 0, 1, 1, 0, 0, 1]);
    show_subtraction(&[1, 0, 1, 1, 1, 0, 1], &[1, 0, 1, 1, 0]);
    show_subtraction(&[1, 0, 1, 0], &[1, 0, 1, 0]);
}
