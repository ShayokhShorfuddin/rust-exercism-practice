pub fn nth(n: u32) -> u32 {
    let mut number: i32 = 2;
    let mut prime_generated_so_far: i32 = 0;

    while prime_generated_so_far - 1 < n as i32 {
        dbg!(prime_generated_so_far, number);

        if is_prime(number) {
            prime_generated_so_far += 1;
        }

        number += 1
    }

    number -= 1;
    number as u32
}

pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }

    let mut number: i32 = 2;

    while number < n {
        if n % number == 0 {
            return false;
        }

        number += 1
    }

    true
}
