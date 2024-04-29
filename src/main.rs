use std::collections::HashMap;
use std::io;

fn main() {
    println!("Enter a number:");
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Failed to read line");
    let input: i128 = input_str.trim().parse().expect("Please type a number!");

    // Step 2: Find the greatest odd number less than or equal to input
    let _greatest_odd = if input % 2 == 0 { input - 1 } else { input };

    // Step 3: Find the greatest odd number less than or equal to the square of input
    let square = input * input;
    let mut closest_odd = if square % 2 == 0 { square - 1 } else { square };

    // Creating HashMap
    let mut hashmap = HashMap::new();
    while closest_odd > 0 {
        hashmap.insert(closest_odd, false);
        closest_odd -= 2;
    }

    // Step 5: Iterate and multiply
    let mut closest_odd = if square % 2 == 0 { square - 1 } else { square };
    while closest_odd > 1 {
        let mut other = closest_odd;
        while other > 1 {
            let product = closest_odd * other;
            if hashmap.contains_key(&product) {
                hashmap.insert(product, true);
            }
            other -= 2;
        }
        closest_odd -= 2;
    }

    // Step 6: Collect and return unmatched numbers
    let mut unmatched: Vec<_> = hashmap.iter()
        .filter(|&(_, &v)| !v)
        .map(|(&k, _)| k)
        .collect();
    
    //sort the vector of unmatched
    unmatched.sort();

    println!("Odd numbers not matched in hashmap:");
    for num in unmatched {
        println!("{}", num);
    }
}

