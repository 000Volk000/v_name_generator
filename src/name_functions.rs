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
    let letters_size = ('z' as u32) - ('a' as u32) + 1;

    for _ in 0..size {
        name.push(((('a' as u32) + (rng.random::<u32>() % letters_size)) as u8).into());
    }

    name
}

pub fn is_valid_name(name: String) -> bool {
    // Minimum number of vowels of the global name
    if how_many_vowels(name.clone()) < ((name.len() as i8 - 1) / 2) {
        return false;
    }

    // Any consonant should be followed by vowel r or l
    let dual_partners = ['a', 'e', 'i', 'o', 'u', 'r', 'l'];
    for i in 0..(name.len() - 1) {
        if !is_vowel(
            name.chars()
                .nth(i)
                .expect("Something wrong happened getting the char i of name"),
        ) && !dual_partners.contains(
            &name
                .chars()
                .nth(i + 1)
                .expect("Something wrong happened getting the char i+1 of name"),
        ) {
            return false;
        }
    }

    // Check for vowel every max 3 letters
    let mut windows = 1;
    for i in 1..name.len() {
        if is_vowel(
            name.chars()
                .nth(i)
                .expect("Something wrong happened getting the char i+1 of name"),
        ) {
            windows = 2
        } else {
            windows -= 1;
        }

        if windows < 0 {
            return false;
        }
    }

    true
}
