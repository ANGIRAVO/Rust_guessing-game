use wasm_bindgen::prelude::*;
use rand::Rng;
use std::cell::RefCell;

thread_local! {
    static SECRET_NUMBER: RefCell<u32> = RefCell::new(rand::thread_rng().gen_range(1..=100));
}

#[wasm_bindgen]
pub fn check_guess(guess: u32) -> String {
    SECRET_NUMBER.with(|secret| {
        let secret = *secret.borrow();
        if guess < secret {
            "Too small!".to_string()
        } else if guess > secret {
            "Too big!".to_string()
        } else {
            "You win!".to_string()
        }
    })
}

#[wasm_bindgen]
pub fn reset_game() {
    SECRET_NUMBER.with(|s| {
        *s.borrow_mut() = rand::thread_rng().gen_range(1..=100);
    });
}
