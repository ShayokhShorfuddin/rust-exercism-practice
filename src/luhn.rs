pub fn luhn(code: &str) -> bool {
    // Remove whitespace
    let trimmed: String = code
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();

    // Check if the input is larger than 1 character or not
    if trimmed.len() <= 1 {
        return false;
    }

    // Reject anything that is not a digit
    if trimmed.chars().any(|character| !character.is_ascii_digit()) {
        return false;
    }

    let sum: u32 = trimmed
        .chars()
        .rev()
        .enumerate()
        // Step 1: Double every other number, starting from the 2nd number from the right
        .filter_map(|(index, character)| match character.to_digit(10) {
            Some(digit) => {
                let doubled = if index % 2 == 1 { digit * 2 } else { digit };
                Some(doubled)
            }
            _ => None,
        })
        // Step 2: Sum the digits of numbers which are 10 or higher
        .map(|number| if number > 9 { number - 9 } else { number })
        // Step 3: Sum all numbers
        .sum();

    // Step 4: Check if the sum is divisible by 10
    sum % 10 == 0
}
