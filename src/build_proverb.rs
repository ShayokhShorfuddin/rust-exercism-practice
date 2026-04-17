pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::from("");

    for (index, _) in list.iter().enumerate() {
        if index + 1 == list.len() {
            proverb += &format!("And all for the want of a {}.", list[0])
        } else {
            proverb += &format!(
                "For want of a {} the {} was lost.\n",
                list[index],
                list[index + 1]
            )
        }
    }

    proverb
}
