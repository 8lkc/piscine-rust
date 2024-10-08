use borrow_box::*;

fn main() {
    let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
    println!("{:#?}", game); println!("{:?}", game.read_winner());

    game.update_score(String::from("Joao"));
    game.update_score(String::from("Joao"));
    game.update_score(String::from("Susana"));
    game.update_score(String::from("Susana"));
    println!("{:?}", game.read_winner());

    game.update_score(String::from("Joao"));
    game.update_score(String::from("Susana"));
    println!("{:?}", game.read_winner());

    println!("{:?}", game.delete());
}
