pub fn is_message_uppercase(message: &str) -> bool {
    let has_alpha = message.chars().any(|c| c.is_alphabetic());
    has_alpha
        && message
            .chars()
            .all(|c| !c.is_alphabetic() || c.is_uppercase())
}

pub fn reply(message: &str) -> &str {
    if message.trim() == "" {
        "Fine. Be that way!"
    } else if message.trim().ends_with("?") {
        if is_message_uppercase(message) {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if is_message_uppercase(message) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
