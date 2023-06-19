pub fn run_example() {
    let player1 = String::from("Player 1");
    let player2 = String::from("Player 2");

    let winner = first_turn(&player1, &player2);
    println!("{} wins!", winner);
}

fn first_turn<'a>(a: &'a str, b: &'a str) -> &'a str {
    match rand::random() {
        true => a,
        false => b,
    }
}
