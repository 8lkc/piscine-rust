use panic::*;

fn main() {
    let file = open_file("test.txt");
    println!("{:?}", file);
}