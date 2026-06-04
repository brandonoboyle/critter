// lib.rs — the game's LOGIC CORE, in Rust.
//
// This owns the state and the rules. It knows NOTHING about pixels, buttons, or
// the screen. wasm-pack compiles it to WebAssembly so TypeScript can call it.
//
// The big idea: the game STATE lives inside the `Game` struct, here in Rust.
// JavaScript never holds the state — it only gets a handle to `Game` and calls
// its methods. So the renderer literally cannot reach in and change the rules.
//
// Two Rust things to notice:
//   - `Option<f64>` is Rust's "a number, or nothing" — it replaces `null`. You
//     must handle the "nothing" case, so you can't forget it.
//   - We never need a garbage collector; the struct owns its data outright.

use wasm_bindgen::prelude::*;

// How long one forage takes, in milliseconds.
// 👉 Tweak this and the bar fills faster or slower.
const FORAGE_DURATION_MS: f64 = 3000.0;

// `#[wasm_bindgen]` on the struct exposes it to JavaScript as a class.
#[wasm_bindgen]
pub struct Game {
    // When the current forage began (a timestamp), or None if idle.
    started_at: Option<f64>,
    // How many berries we've collected. u32 = unsigned 32-bit integer.
    berries: u32,
}

// Methods exposed to JavaScript. wasm-bindgen automatically renames them to
// camelCase on the JS side (e.g. start_foraging -> startForaging).
#[wasm_bindgen]
impl Game {
    // `#[wasm_bindgen(constructor)]` makes this callable as `new Game()` in JS.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            started_at: None,
            berries: 0,
        }
    }

    // Begin foraging. `&mut self` means this method may change the game's state.
    pub fn start_foraging(&mut self, now: f64) {
        // Already foraging? Ignore, so a click can't restart mid-forage.
        if self.started_at.is_none() {
            self.started_at = Some(now);
        }
    }

    // Advance the game to the current moment ("resolve-to-now").
    pub fn resolve(&mut self, now: f64) {
        // `if let Some(started)` runs only when started_at holds a value,
        // and unwraps it into `started` in one step.
        if let Some(started) = self.started_at {
            if now - started >= FORAGE_DURATION_MS {
                self.started_at = None; // back to idle
                self.berries += 1; // reward
                // 👉 Want endless foraging? Use `self.started_at = Some(now);`
                //    instead of setting it to None.
            }
        }
    }

    // How full the progress bar should be: 0.0 (empty) to 1.0 (full).
    // `&self` (no `mut`) means this only READS state — it never changes it.
    pub fn foraging_progress(&self, now: f64) -> f64 {
        // `match` handles both cases of the Option explicitly.
        match self.started_at {
            None => 0.0,
            Some(started) => ((now - started) / FORAGE_DURATION_MS).min(1.0),
        }
    }

    // Read-only getters JavaScript uses to draw the HUD.
    pub fn berries(&self) -> u32 {
        self.berries
    }

    pub fn is_foraging(&self) -> bool {
        self.started_at.is_some()
    }
}
