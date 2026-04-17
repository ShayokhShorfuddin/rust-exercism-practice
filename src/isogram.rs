pub fn check(candidate: &str) -> bool {
    let mut used_characters = vec![];

    for character in candidate.chars() {
        if !character.is_ascii_alphanumeric() {
            continue;
        }

        if used_characters.contains(&character.to_ascii_lowercase()) {
            return false;
        } else {
            used_characters.push(character.to_ascii_lowercase());
        }
    }

    true
}
