# 🎯 Rust Guessing Game (WebAssembly Edition)

This is a simple and interactive number guessing game built with Rust and compiled to WebAssembly (WASM). The frontend is written in HTML, CSS, and JavaScript. Users try to guess a number between 1 and 100, and receive feedback after each guess.

## 🧠 How It Works

- The core game logic is implemented in Rust.
- The Rust code is compiled to WebAssembly using wasm-pack.
- A clean and modern frontend interfaces with the compiled WASM code via JavaScript.

## 📸 Demo

> Guess the number between 1 and 100. Enter your number and click "Guess"! Reset the game anytime.

<!-- Optionally add a screenshot -->
<!-- ![Screenshot](./screenshot.png) -->

## 🛠️ Technologies Used

- 🦀 Rust
- 📦 wasm-pack
- 🕸️ WebAssembly
- 🌐 HTML, CSS (Dark Mode), JavaScript

## 🚀 Getting Started

### Prerequisites

Ensure you have the following installed:

- [Rust](https://rustup.rs)
- wasm-pack:  
  ```bash
  cargo install wasm-pack
  
Clone the Repository
git clone https://github.com/your-username/rust-guessing-game-wasm.git
cd rust-guessing-game-wasm

Build the WASM Package
 ```bash
cd guessing_game_wasm
wasm-pack build --target web
```

Run the Web App
Use any static file server (e.g., http-server or Python's http module). Here's an example using npm's http-server:

```
npm install -g http-server
cd www
http-server -c-1
```
Then open your browser at:
```
http://localhost:8080
```
📁 Project Structure
```
rust-guessing-game-wasm/
│
├── guessing_game_wasm/       # Rust game logic compiled to WASM
│   ├── src/
│   └── Cargo.toml
│
├── www/                      # Frontend code
│   ├── index.html
│   ├── style.css
│   └── index.js
│
├── README.md
```

✨ Features
🎨 Responsive UI with dark mode

⚡ Instant feedback for user guesses

🔁 Reset button to restart the game

🚀 Fast and efficient thanks to WebAssembly

📦 Build Output
After building with wasm-pack, the output is placed in:
```
guessing_game_wasm/pkg/
```
📃 License
This project is licensed under the MIT License. See the LICENSE file for details.








