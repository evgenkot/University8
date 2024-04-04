use std::collections::{HashSet, VecDeque};

pub enum GameStage {
    Start,
    Play,
    End,
}
pub struct GameState {
    pub stage: GameStage,
    pub players: VecDeque<String>,
}


fn main() {
    let mut state = GameState {stage: GameStage::Start, players: vec!["Player1".to_string(), "Player2".to_string()].into_iter().collect()};
    let mut cities: HashSet<String> = HashSet::new();
    let mut current_word = "abobus".to_string();
    cities.insert(current_word.clone());
    let mut last_char = current_word.trim().to_lowercase().chars().last().unwrap();
    loop {
        let mut guess = String::new();
        if state.players.len() <= 1 {
            println!("Thats all {} won!", state.players.pop_front().unwrap_or("NO ONE".to_string()));
        }
        let current_player = state.players.pop_front().unwrap();
        println!("{} turn...", current_player);
        println!("The current word is \"{}\"", current_word);

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.trim().to_lowercase();
        let first_char = guess.chars().next().unwrap();

        if first_char == last_char {
            if cities.contains(&guess) {
                println!("{} already been guessed", guess);
                state.players.push_front(current_player);
                continue;
            }
            current_word = guess.clone();
            cities.insert(guess.clone());
            last_char = guess.chars().last().unwrap();
            state.players.push_back(current_player);
        } else {
            println!("{} not equal {}", last_char, first_char);
            state.players.push_front(current_player);
            continue;
        }
    }
}
