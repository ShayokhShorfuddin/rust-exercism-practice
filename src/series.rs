pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len < 1 {
        return vec![];
    }

    if digits.len() < len {
        return vec![];
    }

    let mut left_pointer = 0;
    let mut substrings: Vec<String> = vec![];

    while left_pointer + len <= digits.len() {
        substrings.push(String::from(&digits[left_pointer..len + left_pointer]));
        left_pointer += 1
    }

    substrings
}
