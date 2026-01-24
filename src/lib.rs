mod name_functions;
use crate::name_functions::{generate_random_name, is_valid_name};
use rand::Rng;

pub fn generate_valid_name() -> String {
    let mut rng = rand::rng();
    let letter_number = 3 + (rng.random::<i8>() % 3).abs();
    let mut name = generate_random_name(letter_number, &mut rng);

    while !is_valid_name(name.clone()) {
        name = generate_random_name(letter_number, &mut rng);
    }

    name
}
