mod utils;

use serde::Deserialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use solver::{Answer, Result, Solver};

#[derive(Deserialize)]
struct Guesses(Vec<(String, usize)>);

fn _solve(words: JsValue, guesses: JsValue) -> Result<Answer> {
    let words: Vec<String> = from_value(words)?;
    let guesses: Guesses = from_value(guesses)?;
    let s = Solver::new(words, guesses.0)?;
    Ok(s.answer())
}

#[wasm_bindgen]
pub fn solve(words: JsValue, guesses: JsValue) -> JsValue {
    match _solve(words, guesses) {
        Ok(answer) => to_value(&answer).unwrap(),
        Err(e) => to_value(&e.to_string()).unwrap(),
    }
}
