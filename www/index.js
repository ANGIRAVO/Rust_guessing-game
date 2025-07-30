import init, { check_guess, reset_game } from "./pkg/guessing_game_wasm.js";

async function run() {
  await init();

  const guessInput = document.getElementById("guess-input");
  const guessBtn = document.getElementById("guess-btn");
  const resetBtn = document.getElementById("reset-btn");
  const resultDiv = document.getElementById("result-message");

  guessBtn.addEventListener("click", () => {
    const guess = parseInt(guessInput.value);
    const result = check_guess(guess);
    resultDiv.textContent = result;
    guessInput.value = "";
  });

  resetBtn.addEventListener("click", () => {
    reset_game();
    resultDiv.textContent = "Game has been reset. Try again!";
    guessInput.value = "";
  });
}

run();
