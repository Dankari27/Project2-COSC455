// Part III: Rust Math Translation

// Translates the words and then shows the sum and product
fn go(input: &[&str]) {
    // number words in Chinese and English
    let chinese: Vec<&str> = vec![
        "ling", "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu", "shi",
    ];
    let english: Vec<&str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];
    // Turn the words into numbers and throw out anything that isn't known
    let numbers = translate_all(input, &chinese, &english);
    println!("Translation: {}", join_numbers(&numbers, " "));
    println!(
        "Addition: {} = {}",
        join_numbers(&numbers, " + "),
        sum_list(&numbers)
    );
    println!(
        "Multiplication: {} = {}",
        join_numbers(&numbers, " * "),
        product_list(&numbers)
    );
}

//finds where the word is in the list and gives back its spot as the number
fn find_index(word: &str, list: &[&str], index: i32) -> Option<i32> {
    match list {
        // Went through the whole list and never found the word
        [] => None,
        // the front word matches so this spot is the number
        [first, ..] if *first == word => Some(index),
        // not a match so keep checking the rest and move the spot up one
        [_, rest @ ..] => find_index(word, rest, index + 1),
    }
}

// Turns a Chinese or English word into its number
fn translate_word(word: &str, chinese: &[&str], english: &[&str]) -> Option<i32> {
    match find_index(word, chinese, 0) {
        //It was a Chinese word so use that number
        Some(n) => Some(n),
        // not Chinese so check the English list and gives None if it isn't there either
        None => find_index(word, english, 0),
    }
}

// Translates every word in the list and drops the ones it doesn't know
fn translate_all(words: &[&str], chinese: &[&str], english: &[&str]) -> Vec<i32> {
    match words {
        // Nothing left to translate
        [] => vec![],
        // Translate the front word and recurse on the rest
        [first, rest @ ..] => match translate_word(first, chinese, english) {
            // Known word so keep its number in front
            Some(n) => [vec![n], translate_all(rest, chinese, english)].concat(),
            // Unknown word so skip it and keep going
            None => translate_all(rest, chinese, english),
        },
    }
}

// Adds up every number in the list
fn sum_list(numbers: &[i32]) -> i32 {
    match numbers {
        // An empty list adds up to 0
        [] => 0,
        // add the front number to the sum of the rest
        [first, rest @ ..] => first + sum_list(rest),
    }
}

// Multiplies every number in the list together
fn product_list(numbers: &[i32]) -> i32 {
    match numbers {
        // An empty list multiplies out to 1
        [] => 1,
        // multiply the front number by the product of the rest
        [first, rest @ ..] => first * product_list(rest),
    }
}

// Turns a list of numbers into a string with the separator between each one
fn join_numbers(numbers: &[i32], separator: &str) -> String {
    match numbers {
        // Nothing left to join
        [] => String::new(),
        // Last number so no separator after it
        [last] => last.to_string(),
        // Put the front number and separator in the string and recurse on the rest
        [first, rest @ ..] => first.to_string() + separator + &join_numbers(rest, separator),
    }
}

fn main() {
    // examples from the assignment
    let input = vec!["yi", "nine", "six", "ba"];
    go(&input);
    println!();

    // josh should get filtered out
    let input = vec!["yi", "josh", "three", "si"];
    go(&input);
}
