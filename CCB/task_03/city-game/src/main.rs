use std::collections::HashSet;

pub enum GameStage {
    Start,
    Play,
    End,
}
pub struct GameState {
    pub stage: GameStage,
    pub players: Vec<String>,
}


fn main() {
    let mut cities: HashSet<String> = HashSet::new();
    let mut current_word = "abobus".to_string();
    cities.insert(current_word.clone());
    let mut last_char = current_word.trim().to_lowercase().chars().last().unwrap();
    loop {
        let mut guess = String::new();
        println!("The current word is \"{}\"", current_word);

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.trim().to_lowercase();
        let first_char = guess.chars().next().unwrap();

        if first_char == last_char {
            if cities.contains(&guess) {
                println!("{} already been guessed", guess);
                continue;
            }
            current_word = guess.clone();
            cities.insert(guess.clone());
            last_char = guess.chars().last().unwrap();
        } else {
            println!("{} not equal {}", last_char, first_char);
            continue;
        }
    }
}
