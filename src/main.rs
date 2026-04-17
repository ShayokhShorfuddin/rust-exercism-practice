mod build_proverb;

fn main() {
    println!(
        "{}",
        build_proverb::build_proverb(&[
            "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
        ])
    );
}
