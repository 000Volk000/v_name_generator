use rand::{Rng, rngs::ThreadRng};

fn generate_random_name(size: i8, mut rng: ThreadRng) -> String {
    let mut name = String::new();
    name.push('V');
    let letters_size = ('z' as u32) - ('a' as u32);

    for _ in 0..size {
        name.push(((('a' as u32) + (rng.random::<u32>() % letters_size)) as u8).into());
    }

    name
}

fn main() {
    let mut rng = rand::rng();
    let letter_number = 3 + (rng.random::<i8>() % 3).abs();
    println!("{}", generate_random_name(letter_number, rng));
}
