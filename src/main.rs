mod acronym;

fn main() {
    // let input = "Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me";
    let input = "Something - I made up from thin air";
    // let input = "HyperText Markup Language";
    dbg!(acronym::abbreviate(input));
}
