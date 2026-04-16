pub fn sum(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

pub fn square_of_sum(n: u32) -> u32 {
    sum(n).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut squares: Vec<u32> = vec![];

    for number in 1..=n {
        squares.push(number.pow(2));
    }

    squares.iter().sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
