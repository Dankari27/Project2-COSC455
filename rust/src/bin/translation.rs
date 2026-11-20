// Part III: Rust Math Translation
#![allow(dead_code, unused_variables)] // Hides warnings while things are still unfinished

// Translates the words and then shows the sum and product
fn go(input: &[&str]) {
    todo!()
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

fn main() {
    // number words in Chinese and English
    let chinese: Vec<&str> = vec![
        "ling", "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu", "shi",
    ];
    let english: Vec<&str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];

    // example from the assignment
    let input = vec!["yi", "nine", "six", "ba"];
    go(&input);
}
