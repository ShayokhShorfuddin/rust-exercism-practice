mod hamming;

fn main() {
    dbg!(hamming::hamming_distance(
        "GAGCCTACTAACGGGAT",
        "CATCGTAATGACGGCCT"
    ));
}
