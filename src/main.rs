mod tournament;

fn main() {
    let input: &[&str] = &[
        "Devastating Donkeys;Blithering Badgers;win",
        "Devastating Donkeys;Blithering Badgers;win",
        "Devastating Donkeys;Blithering Badgers;win",
        "Devastating Donkeys;Blithering Badgers;win",
        "Blithering Badgers;Devastating Donkeys;win",
    ];
    let input = input.join("\n");

    let result = tournament::tally(&input);

    println!("{}", result);
}
