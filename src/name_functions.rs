use rand::{Rng, rngs::ThreadRng};

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

fn is_vowel(letter: char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    vowels.contains(&letter)
}

pub fn generate_random_name(size: i8, rng: &mut ThreadRng) -> String {
    let mut name = String::new();
    name.push('V');
    let letters_size = ('z' as u32) - ('a' as u32);

    for _ in 0..size {
        name.push(((('a' as u32) + (rng.random::<u32>() % letters_size)) as u8).into());
    }

    name
}

pub fn is_valid_name(name: String) -> bool {
    if how_many_vowels(name.clone()) < ((name.len() as i8 - 1) / 2) {
        return false;
    }

    true
}
