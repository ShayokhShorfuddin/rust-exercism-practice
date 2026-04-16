pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps: u64 = 0;
    let mut number = n;

    while number != 1 {
        if number % 2 == 0 {
            number /= 2;
        } else {
            number = (number * 3) + 1
        }

        steps += 1
    }

    Some(steps)
}
