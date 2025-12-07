use rand::{Rng, rngs::ThreadRng};

fn generate_random_name(size: i8, rng: &mut ThreadRng) -> String {
    let mut name = String::new();
    name.push('V');
    let letters_size = ('z' as u32) - ('a' as u32);

    for _ in 0..size {
        name.push(((('a' as u32) + (rng.random::<u32>() % letters_size)) as u8).into());
    }

    name
}

fn how_many_vowels(name: String) -> i8 {
    let mut n = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for letter in name.chars() {
        if vowels.contains(&letter) {
            n += 1;
        }
    }

    n
}

fn main() {
    let mut rng = rand::rng();
    let letter_number = 3 + (rng.random::<i8>() % 3).abs();
    let mut name = generate_random_name(letter_number, &mut rng);

    while how_many_vowels(name.clone()) < (letter_number / 2) {
        name = generate_random_name(letter_number, &mut rng);
    }

    println!("{}", name);
}
