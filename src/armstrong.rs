// Idiomatic solution
pub fn idiomatic_is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|char_of_digit| {
            char_of_digit
                .to_digit(10)
                .expect("to_string couldn't produce valid digits")
        })
        .collect();

    let power_to_be_raised_to = digits.len() as u32;

    digits
        .iter()
        .map(|digit| digit.pow(power_to_be_raised_to))
        .sum::<u32>()
        == num
}

// My attempt
pub fn is_armstrong_number(num: u32) -> bool {
    let string_of_num = num.to_string();
    let number_of_digits = string_of_num.len() as u32;

    let mut raised_digits: Vec<u32> = vec![];

    // Raise every digit in the given number to the number of digits
    for digit in string_of_num.chars() {
        let int_of_digit = digit.to_digit(10);

        match int_of_digit {
            Some(value) => raised_digits.push(value.pow(number_of_digits)),
            None => panic!("Could not convert char to digit"),
        };
    }

    let sum: u32 = raised_digits.iter().sum();

    sum == num
}
