// Part VII: Drunk Homer (Extra Credit)

//the stores homer remembers going to
const STORES: [&str; 4] = ["The Leftorium", "Sprawl-Mart", "Try-N-Save", "King Toots"];

// The gifts homer bought
const GIFTS: [&str; 4] = ["green dress", "saxophone book", "slingshot", "pacifier"];

// builds every possible ordering of the items
fn permutations(items: &[&'static str]) -> Vec<Vec<&'static str>> {
    match items {
        [] => vec![vec![]],
        // Pick each item to go first and then put it in front of every ordering of the rest
        _ => items
            .iter()
            .enumerate()
            .flat_map(|(i, first)| {
                //everything except the item picked to go first
                let rest: Vec<&str> = items
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, item)| *item)
                    .collect();
                permutations(&rest)
                    .into_iter()
                    .map(move |perm| [vec![*first], perm].concat())
            })
            .collect(),
    }
}

// finds which stop number an item is at
fn position(list: &[&str], item: &str) -> Option<usize> {
    list.iter().position(|x| *x == item)
}

// Clue 1: He bought the saxophone book at king toots
fn clue_one(stores: &[&str], gifts: &[&str]) -> bool {
    // The saxophone book and king Toots have to be at the stop
    position(gifts, "saxophone book") == position(stores, "King Toots")
}

// Clue 2: The store he visited immediately after buying the slingshot was not sprawl mart
fn clue_two(stores: &[&str], gifts: &[&str]) -> bool {
    match position(gifts, "slingshot") {
        //there is a stop after the slingshot so it can't be sprawl mart
        Some(i) if i + 1 < stores.len() => stores[i + 1] != "Sprawl-Mart",
        _ => false,
    }
}

// Clue 3:The leftorium was his second stop
fn clue_three(stores: &[&str]) -> bool {
    // index 1 is the second stop since the list starts at 0
    position(stores, "The Leftorium") == Some(1)
}

// Clue 4: Two stops after leaving try n save he then bought the pacifier
fn clue_four(stores: &[&str], gifts: &[&str]) -> bool {
    match (position(stores, "Try-N-Save"), position(gifts, "pacifier")) {
        // the pacifier has to be exactly two stops after try n save
        (Some(t), Some(p)) => p == t + 2,
        _ => false,
    }
}

// Checks if an ordering of stores and gifts makes all four clues true
fn solves_puzzle(stores: &[&str], gifts: &[&str]) -> bool {
    clue_one(stores, gifts) && clue_two(stores, gifts) && clue_three(stores) && clue_four(stores, gifts)
}

//Tries every store ordering with every gift ordering 
fn solve() -> Vec<(Vec<&'static str>, Vec<&'static str>)> {
    let store_orders = permutations(&STORES);
    let gift_orders = permutations(&GIFTS);
    store_orders
        .iter()
        .flat_map(|stores| {
            gift_orders
                .iter()
                .filter(move |gifts| solves_puzzle(stores, gifts))
                .map(move |gifts| (stores.clone(), gifts.clone()))
        })
        .collect()
}

// Prints each stop with the store and the gift bought there
fn print_solution(stores: &[&str], gifts: &[&str]) {
    stores
        .iter()
        .zip(gifts.iter())
        .enumerate()
        .for_each(|(i, (store, gift))| println!("Stop {}: {} - bought the {}", i + 1, store, gift));
}

fn main() {
    let solutions = solve();
    println!("Orderings checked: {}", permutations(&STORES).len() * permutations(&GIFTS).len());
    println!("Solutions found: {}\n", solutions.len());
    match solutions.as_slice() {
        // No solution
        [] => println!("Homer must have been too drunk, no solution works"),
        // Print every solution that was found
        _ => solutions.iter().for_each(|(stores, gifts)| print_solution(stores, gifts)),
    }
}
// I hate Homer. 