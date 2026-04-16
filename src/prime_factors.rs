pub fn factors(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = vec![];
    let mut divisor: u64 = 2;
    let mut number = n;

    while number != 1 {
        if number % divisor == 0 {
            number /= divisor;
            primes.push(divisor);
        } else {
            divisor += 1;
        }
    }

    primes
}
