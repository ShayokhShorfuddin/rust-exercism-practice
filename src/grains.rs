pub fn square(s: u32) -> u64 {
    let mut grain_state = 1;

    if s == 0 {
        panic!()
    }

    if s == 1 {
        return 1;
    }

    for _ in 2..=s {
        grain_state *= 2
    }

    grain_state
}

pub fn total() -> u64 {
    (1..=64).map(square).sum::<u64>()
}
