mod name_functions;
use dotenv::dotenv;
use rand::Rng;
use std::env;

use crate::name_functions::{generate_random_name, is_valid_name};

fn main() {
    let mut rng = rand::rng();
    let letter_number = 3 + (rng.random::<i8>() % 3).abs();
    let mut name = generate_random_name(letter_number, &mut rng);

    while !is_valid_name(name.clone()) {
        name = generate_random_name(letter_number, &mut rng);
    }

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args.get(1).expect("Error geting the 1 arg") == "ntfy" {
        dotenv().expect("Couldn load the .env");
        let ntfy = env::var("NTFY_POST_URL").expect("Couldn't find the NTFY_POST_URL on the .env");

        println!("{}", ntfy)
    } else {
        println!("{}", name);
    }
}
