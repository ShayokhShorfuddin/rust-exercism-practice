// Idiomatic way
pub fn idiom_reverse(input: &str) -> String {
    input.chars().rev().collect()
}

// My attempt
pub fn reverse(input: &str) -> String {
    let mut reversed = String::from("");
    let characters: Vec<char> = input.chars().collect();

    let mut count = characters.len();

    while count != 0 {
        count -= 1;
        reversed.push(characters[count]);
    }

    reversed
}
