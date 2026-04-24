pub fn abbreviate(phrase: &str) -> String {
    let mut output = String::new();

    if phrase.trim().is_empty() {
        return String::new();
    }

    let words: Vec<&str> = phrase.split(" ").collect();

    for word in words {
        if word.contains('-') {
            for sub_word in word.split("-").collect::<Vec<&str>>() {
                if sub_word.is_empty() {
                    continue;
                }

                // If it's an all caps word (like GNU), we only add the first capital (G in this case)
                if is_all_caps_word(sub_word) {
                    output += &sub_word[0..1];
                } else if is_all_lowercase_word(sub_word) {
                    output += &sub_word[0..1].to_uppercase();
                } else {
                    output += &String::from_iter(sub_word.chars().filter(|character| {
                        character.is_ascii_alphabetic() && character.is_uppercase()
                    }));
                }
            }
        } else {
            // If it's an all caps word (like GNU), we only add the first capital (G in this case)
            if is_all_caps_word(word) {
                output += &word[0..1];
            } else if is_all_lowercase_word(word) {
                output += &word[0..1].to_uppercase();
            } else {
                output += &String::from_iter(word.chars().filter(|character| {
                    character.is_ascii_alphabetic() && character.is_uppercase()
                }));
            }
        }
    }

    output
}

fn is_all_caps_word(word: &str) -> bool {
    let capitals_in_word: Vec<char> = word
        .chars()
        .filter(|character| character.is_ascii_alphabetic() && character.is_uppercase())
        .collect();

    word.len() == capitals_in_word.len()
}

fn is_all_lowercase_word(word: &str) -> bool {
    let lowercase_in_word: Vec<char> = word
        .chars()
        .filter(|character| character.is_ascii_alphabetic() && character.is_lowercase())
        .collect();

    word.len() == lowercase_in_word.len()
}

// String::from_iter(
//     phrase
//         .chars()
//         .filter(|char| char.is_ascii_alphabetic() && char.is_uppercase()),
// )
