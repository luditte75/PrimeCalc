use std::collections::HashMap;
use std::env;

fn main() {
    // Read the maximum number from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <Max_Num>", args[0]);
        return;
    }

    let max_num: i32 = args[1].parse().expect("Please provide an integer for Max_Num");

    // Create a hashmap with keys incrementing by odd numbers
    let mut hashmap = HashMap::new();
    let mut current_number = 1;
    while current_number <= max_num {
        hashmap.insert(current_number, false);  // false means no skip flag
        current_number += 2;
    }

    // Calculate M_Start by bitshifting the highest key
    let mut m_start = *hashmap.keys().max().unwrap() >> 1; // Bitshift right by 1 divides by 2

    // Perform the filtering as specified
    while m_start >= 1 {
        for key in hashmap.keys().copied().collect::<Vec<_>>() {
            let product = key * m_start;
            if product <= max_num {
                if hashmap.contains_key(&product) {
                    hashmap.insert(product, true); // Set skip flag to true
                }
            }
        }
        m_start >>= 1; // Continue dividing M_Start by 2
    }

    // Collect and print numbers that were not skipped
    let results: Vec<String> = hashmap.iter()
        .filter_map(|(&key, &value)| if !value { Some(key.to_string()) } else { None })
        .collect();
    println!("{}", results.join(", "));
}
