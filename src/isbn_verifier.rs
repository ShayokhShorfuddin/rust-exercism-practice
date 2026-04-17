/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let filtered: String = isbn.chars().filter(|character| *character != '-').collect();

    if filtered.len() != 10 {
        return false;
    }

    let mut sum = 0;

    for number in 1..=10 {
        let character = filtered
            .chars()
            .nth(number - 1)
            .expect("Failed to get the expected number from the ISBN chars");

        let isbn_digit;

        if number == 10 {
            if character.is_numeric() {
                match character.to_digit(10) {
                    Some(digit) => isbn_digit = digit,
                    None => return false,
                }
            } else if character == 'X' {
                isbn_digit = 10;
            } else {
                return false;
            }
        } else {
            match character.to_digit(10) {
                Some(digit) => isbn_digit = digit,
                None => return false,
            }
        }

        sum += isbn_digit * (number as u32)
    }
    sum % 11 == 0
}
