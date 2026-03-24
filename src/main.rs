// mod hello_world;
mod reverse_string;

fn main() {
    // hello_world::hello_world();
    let output = reverse_string::reverse("input");
    println!("{}", output)
}
