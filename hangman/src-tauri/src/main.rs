// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// ============================================================================
// DEPENDENCY IMPORTS (External Crates)
// ============================================================================
// These "use" statements import functionality from external Rust libraries (crates).
// 
// HOW DEPENDENCIES ARE INSTALLED:
// 1. Dependencies are declared in Cargo.toml under [dependencies]
// 2. When you run "cargo build" or "cargo run", Cargo automatically:
//    - Downloads the crates from crates.io (Rust's package registry)
//    - Compiles them
//    - Links them to your project
// 3. The compiled dependencies are cached in the "target/" directory
//
// IMPORTED CRATES:

// rand::Rng - Random number generation trait
// Source: "rand = "0.8"" in Cargo.toml
// Used for: Randomly selecting a word from our word list
use rand::Rng;

// serde - Serialization/Deserialization framework
// Source: "serde = { version = "1.0", features = ["derive"] }" in Cargo.toml
// Used for: Converting Rust structs to/from JSON for communication with the frontend
// The Deserialize and Serialize traits allow automatic JSON conversion
use serde::{Deserialize, Serialize};

// std::sync::Mutex - Standard library (built-in, no Cargo.toml entry needed)
// Source: Rust standard library (always available)
// Used for: Thread-safe access to shared game state across multiple requests
use std::sync::Mutex;

// tauri::State - Tauri framework's state management
// Source: "tauri = { version = "1.5", features = ["shell-open"] }" in Cargo.toml
// Used for: Accessing shared application state in Tauri command handlers
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameState {
    word: String,
    guessed_letters: Vec<char>,
    wrong_guesses: u32,
    max_wrong_guesses: u32,
    game_over: bool,
    won: bool,
}

impl GameState {
    fn new(word: String) -> Self {
        GameState {
            word: word.to_uppercase(),
            guessed_letters: Vec::new(),
            wrong_guesses: 0,
            max_wrong_guesses: 6,
            game_over: false,
            won: false,
        }
    }

    fn get_display_word(&self) -> String {
        self.word
            .chars()
            .map(|c| {
                if self.guessed_letters.contains(&c) {
                    c.to_string()
                } else {
                    "_".to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn check_win(&self) -> bool {
        self.word
            .chars()
            .all(|c| self.guessed_letters.contains(&c))
    }
}

struct AppState {
    game: Mutex<GameState>,
}

#[tauri::command]
fn start_new_game(state: State<AppState>) -> GameState {
    let words = vec![
        "RUST",
        "TAURI",
        "PROGRAMMING",
        "COMPUTER",
        "KEYBOARD",
        "DEVELOPER",
        "SOFTWARE",
        "ALGORITHM",
        "FUNCTION",
        "VARIABLE",
        "HANGMAN",
        "CHALLENGE",
        "VICTORY",
        "PUZZLE",
        "MYSTERY",
    ];

    let mut rng = rand::thread_rng();
    let word = words[rng.gen_range(0..words.len())].to_string();

    let new_game = GameState::new(word);
    let mut game = state.game.lock().unwrap();
    *game = new_game.clone();

    new_game
}

#[tauri::command]
fn guess_letter(letter: String, state: State<AppState>) -> GameState {
    let mut game = state.game.lock().unwrap();

    if game.game_over {
        return game.clone();
    }

    let letter_upper = letter.to_uppercase().chars().next().unwrap_or(' ');

    if !game.guessed_letters.contains(&letter_upper) {
        game.guessed_letters.push(letter_upper);

        if !game.word.contains(letter_upper) {
            game.wrong_guesses += 1;
        }

        if game.wrong_guesses >= game.max_wrong_guesses {
            game.game_over = true;
            game.won = false;
        } else if game.check_win() {
            game.game_over = true;
            game.won = true;
        }
    }

    game.clone()
}

#[tauri::command]
fn get_game_state(state: State<AppState>) -> GameState {
    let game = state.game.lock().unwrap();
    game.clone()
}

fn main() {
    let words = vec!["RUST"];
    let initial_word = words[0].to_string();
    let initial_game = GameState::new(initial_word);

    tauri::Builder::default()
        .manage(AppState {
            game: Mutex::new(initial_game),
        })
        .invoke_handler(tauri::generate_handler![
            start_new_game,
            guess_letter,
            get_game_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
