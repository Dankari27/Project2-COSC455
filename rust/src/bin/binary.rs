// Part II: Binary Addition/Subtraction
#![allow(dead_code, unused_variables)] // Hides warnings while things are still unfinished

// Adds two lists of 1s and 0s
fn binary_addition(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!()
}

// Does the main adding work bit by bit
// Both lists are expected to already be reversed so the least important bit is first
fn do_binary_addition(a: &[bool], b: &[bool], carry: bool) -> Vec<bool> {
    match (a, b) {
        // Both lists ran out, so only the carry might be left over
        ([], []) => finish_binary_add(&[], carry),
        // One list ran out, so pass the other list and the carry along the last
        ([], rest) | (rest, []) => finish_binary_add(rest, carry),
        // Both lists still have bits, so add the two front bits along with the carry
        ([x, xs @ ..], [y, ys @ ..]) => {
            // Pattern match on the three bits to figure out what the sum and new carry are
            let (sum, new_carry) = match (*x, *y, carry) {
                (false, false, false) => (false, false), // 0 + 0 + 0 = 0
                (true, false, false)
                | (false, true, false)
                | (false, false, true) => (true, false), // only one bit is set, so 1
                (true, true, false)
                | (true, false, true)
                | (false, true, true) => (false, true), // two bits are set, so 10
                (true, true, true) => (true, true),    // all three bits are set, so 11
            };
            // Put the sum bit in front and recurse on the rest of both lists
            [vec![sum], do_binary_addition(xs, ys, new_carry)].concat()
        }
    }
}

// Finishes up when one list runs out before the other
fn finish_binary_add(rest: &[bool], carry: bool) -> Vec<bool> {
    match (rest, carry) {
        // Nothing left and no carry, finish
        ([], false) => vec![],
        // Nothing left but there is still a carry
        ([], true) => vec![true],
        // no carry anymore, so the rest of the bits are copied
        (bits, false) => bits.to_vec(),
        // A 0 plus the carry becomes 1 and then stops
        ([false, tail @ ..], true) => [vec![true], tail.to_vec()].concat(),
        // A 1 plus the carry becomes 0 and the carry continues
        ([true, tail @ ..], true) => [vec![false], finish_binary_add(tail, true)].concat(),
    }
}

// Subtracts the second list from the first
fn binary_subtraction(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!()
}

fn main() {
    // Setup the functions later
}
