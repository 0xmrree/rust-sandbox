let currentGameState = null;

// Helper function to invoke Tauri commands
async function invoke(cmd, args = {}) {
    return window.__TAURI__.invoke(cmd, args);
}

// Hangman parts in order
const hangmanParts = [
    'head',
    'body',
    'left-arm',
    'right-arm',
    'left-leg',
    'right-leg'
];

// Initialize the game
async function initGame() {
    createKeyboard();
    await startNewGame();
}

// Create keyboard buttons
function createKeyboard() {
    const keyboard = document.getElementById('keyboard');
    const letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
    
    for (let letter of letters) {
        const button = document.createElement('button');
        button.className = 'key';
        button.textContent = letter;
        button.onclick = () => guessLetter(letter);
        button.id = `key-${letter}`;
        keyboard.appendChild(button);
    }
}

// Start a new game
async function startNewGame() {
    try {
        currentGameState = await invoke('start_new_game');
        updateUI();
        resetHangman();
        enableAllKeys();
        document.getElementById('game-status').textContent = '';
        document.getElementById('game-status').className = 'game-status';
    } catch (error) {
        console.error('Error starting new game:', error);
    }
}

// Guess a letter
async function guessLetter(letter) {
    if (currentGameState.game_over) {
        return;
    }

    try {
        currentGameState = await invoke('guess_letter', { letter });
        updateUI();
        disableKey(letter);
        
        if (currentGameState.game_over) {
            disableAllKeys();
            showGameResult();
        }
    } catch (error) {
        console.error('Error guessing letter:', error);
    }
}

// Update the UI
function updateUI() {
    // Update word display
    const wordDisplay = document.getElementById('word-display');
    wordDisplay.textContent = currentGameState.word
        .split('')
        .map(char => currentGameState.guessed_letters.includes(char) ? char : '_')
        .join(' ');
    
    // Update wrong guesses count
    document.getElementById('wrong-count').textContent = currentGameState.wrong_guesses;
    
    // Update guessed letters
    const guessedList = document.getElementById('guessed-list');
    guessedList.innerHTML = '';
    currentGameState.guessed_letters.forEach(letter => {
        const span = document.createElement('span');
        span.className = 'guessed-letter';
        span.textContent = letter;
        
        if (currentGameState.word.includes(letter)) {
            span.classList.add('correct');
        } else {
            span.classList.add('wrong');
        }
        
        guessedList.appendChild(span);
    });
    
    // Update hangman drawing
    updateHangman(currentGameState.wrong_guesses);
}

// Update hangman drawing
function updateHangman(wrongGuesses) {
    for (let i = 0; i < hangmanParts.length; i++) {
        const part = document.getElementById(hangmanParts[i]);
        if (i < wrongGuesses) {
            part.classList.add('visible');
        } else {
            part.classList.remove('visible');
        }
    }
}

// Reset hangman drawing
function resetHangman() {
    hangmanParts.forEach(partId => {
        document.getElementById(partId).classList.remove('visible');
    });
}

// Show game result
function showGameResult() {
    const statusDiv = document.getElementById('game-status');
    
    if (currentGameState.won) {
        statusDiv.textContent = '🎉 YOU WON! 🎉';
        statusDiv.className = 'game-status win';
    } else {
        statusDiv.textContent = `😢 YOU LOST! The word was: ${currentGameState.word}`;
        statusDiv.className = 'game-status lose';
    }
}

// Disable a specific key
function disableKey(letter) {
    const key = document.getElementById(`key-${letter}`);
    if (key) {
        key.disabled = true;
    }
}

// Disable all keys
function disableAllKeys() {
    const keys = document.querySelectorAll('.key');
    keys.forEach(key => key.disabled = true);
}

// Enable all keys
function enableAllKeys() {
    const keys = document.querySelectorAll('.key');
    keys.forEach(key => key.disabled = false);
}

// Event listeners
document.getElementById('new-game-btn').addEventListener('click', startNewGame);

// Keyboard support
document.addEventListener('keydown', (e) => {
    if (currentGameState && !currentGameState.game_over) {
        const letter = e.key.toUpperCase();
        if (/^[A-Z]$/.test(letter) && !currentGameState.guessed_letters.includes(letter)) {
            guessLetter(letter);
        }
    }
});

// Initialize when page loads
window.addEventListener('DOMContentLoaded', initGame);
