use std::io;

fn main() {
    let vowels = vec!['a', 'e', 'o', 'u', 'i'];

    let mut words = String::new();

    io::stdin().read_line(&mut words).expect("Failed to read line");
    println!("{}", words);

    let mut new_words = String::new();
    let mut iter = words.split_whitespace();
    for mut word in iter {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap().clone();
        // to_lowercase return ToLowercase
        // ToLowercase Iterator Trait Implementations
        // next return Option
        // Option unwrap get value
        let lower_first_char = first_char.to_lowercase().next().unwrap();
        if vowels.contains(&lower_first_char) {
            new_words += &format!("{}-hay ", word);
        } else {
            new_words += &format!("{}-{}ay ", chars.as_str(), &first_char);
        }
    }

    println!("{}", new_words);
}
