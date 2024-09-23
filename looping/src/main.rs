use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut number_of_trials = 0;
    loop {
        number_of_trials += 1;
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        io::stdin().read_line(&mut input)?;
        if input.trim() != "The letter e" {
            number_of_trials += 1;
            println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
            println!("The letter e\nNumber of trials: {}", number_of_trials); break;
        }
    }
    Ok(())
}
