mod luhn;

fn main() {
    let is_valid = luhn::luhn("59");
    println!("{}", is_valid);
}
