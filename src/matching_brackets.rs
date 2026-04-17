use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    let pairs: HashMap<char, char> = HashMap::from([('[', ']'), ('{', '}'), ('(', ')')]);

    for current_character in string.chars() {
        if pairs.contains_key(&current_character) {
            stack.push(current_character);
        }

        if pairs
            .values()
            .any(|value_character| value_character == &current_character)
        {
            if let Some(last_character) = stack.last() {
                if let Some(value) = pairs.get(last_character) {
                    if value == &current_character {
                        match stack.pop() {
                            Some(_) => (),
                            None => return false,
                        }
                    } else {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }
    }

    stack.len() == 0
}
