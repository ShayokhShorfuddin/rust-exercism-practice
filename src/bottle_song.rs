pub fn bottle_song(start_bottles: u32, take_down: u32) -> String {
    let words_of_numbers = [
        "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];

    let mut lyrics = String::from("");

    for number in 0..take_down {
        let current_bottles = (start_bottles - number) as usize;

        let first_two_line = format!(
            "{} green {} hanging on the wall,\n",
            words_of_numbers[current_bottles],
            if current_bottles > 1 {
                "bottles"
            } else {
                "bottle"
            }
        )
        .repeat(2);

        lyrics += &format!(
            "{}And if one green bottle should accidentally fall,\nThere'll be {} green {} hanging on the wall.\n\n",
            first_two_line,
            if current_bottles - 1 == 0 {
                "no".to_string()
            } else {
                words_of_numbers[current_bottles - 1].to_lowercase()
            },
            if current_bottles - 1 > 1 {
                "bottles"
            } else if current_bottles - 1 == 1 {
                "bottle"
            } else {
                "bottles"
            }
        );
    }

    lyrics
}
