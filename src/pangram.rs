/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut alphabets: Vec<char> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    for character in sentence.chars() {
        if alphabets.contains((&character.to_ascii_lowercase())) {
            if let Some(index) = alphabets
                .iter()
                .position(|&current_character| current_character == character.to_ascii_lowercase())
            {
                alphabets.remove(index);
            }
        }
    }

    alphabets.len() == 0
}
