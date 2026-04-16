mod bob;

fn main() {
    println!("{}", bob::is_message_uppercase("1,2,3 GO!"));
    println!("{}", bob::reply("1,2,3 GO!"))
}
