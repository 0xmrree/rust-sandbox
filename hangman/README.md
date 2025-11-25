# Hangman Game

A classic Hangman game built with Rust and Tauri, featuring a modern web-based GUI.

## Features

- 🎮 Classic Hangman gameplay
- 🎨 Beautiful, modern UI with smooth animations
- 🎯 15 hard-coded words to guess
- 🖱️ Click letters or use your keyboard to guess
- 📊 Visual hangman drawing that builds with wrong guesses
- ✅ Win/Lose status display
- 🔄 Easy restart with "New Game" button

## How to Play

1. The game randomly selects a word from a predefined list
2. Guess letters by clicking on the keyboard or typing on your physical keyboard
3. Each wrong guess adds a part to the hangman
4. You have 6 wrong guesses before you lose
5. Guess all letters correctly to win!

## Running the Game

### Prerequisites

- Rust (latest stable version)
- Node.js and npm (for Tauri)

### Development Mode

```bash
cargo tauri dev
```

### Build for Production

```bash
cargo tauri build
```

## Technology Stack

- **Backend**: Rust
- **Frontend**: HTML, CSS, JavaScript
- **Framework**: Tauri 1.5
- **Random Selection**: rand crate

## Word List

The game includes 15 programming-related words:
- RUST
- TAURI
- PROGRAMMING
- COMPUTER
- KEYBOARD
- DEVELOPER
- SOFTWARE
- ALGORITHM
- FUNCTION
- VARIABLE
- HANGMAN
- CHALLENGE
- VICTORY
- PUZZLE
- MYSTERY

Feel free to add more words by editing the `words` vector in `src/main.rs`!

## License

MIT
