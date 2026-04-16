/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) {
    // TODO: Needs more thoughts. I feel like there should be a simpler way

    let filtered_and_reversed: String = isbn
        .chars()
        .filter(|character| *character != '-')
        .rev()
        .collect();

    let number_to_multiply_with = 10;

    let mut result = 0;

    while number_to_multiply_with > 0 {
        let converted_digit = filtered_and_reversed
            .chars()
            .nth(number_to_multiply_with)
            .expect("Haha");
    }
}
