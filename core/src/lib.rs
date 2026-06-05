// lib.rs — the game's LOGIC CORE, in Rust.
//
// This owns the state and the rules. It knows NOTHING about pixels or buttons.
// The big idea is unchanged: the game STATE lives inside `Game`, here in Rust.
// JavaScript only gets a handle and calls methods — it cannot reach in and edit.
//
// What changed: the engine is no longer about "foraging." It runs whatever
// skill is active, drawn from the data table in skills.rs. Foraging is now just
// SKILLS[0]; nothing here mentions berries by name.

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// Pull in the content table (skills.rs in this same crate).
mod skills;
use skills::SKILLS;

// What's currently running: which skill (an index into SKILLS) and when it began.
// Private — JS never sees this struct, only the numbers we expose below.
struct Active {
    skill: usize,
    started_at: f64,
}

#[wasm_bindgen]
pub struct Game {
    // The active skill, or None if the critter is idle.
    active: Option<Active>,
    // Everything we own: item id -> how many. Replaces the old `berries` field.
    inventory: HashMap<String, u32>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            active: None,
            inventory: HashMap::new(),
        }
    }

    // --- Menu data ---------------------------------------------------------
    // JS reads these to BUILD the menu (one button per skill). The front end
    // never hardcodes "Forage" — it asks how many skills exist and their names.

    pub fn skill_count(&self) -> usize {
        SKILLS.len()
    }

    pub fn skill_name(&self, i: usize) -> String {
        SKILLS[i].name.to_string()
    }

    pub fn skill_output(&self, i: usize) -> String {
        SKILLS[i].output.to_string()
    }

    // --- Actions -----------------------------------------------------------

    // Start skill number `i`. Ignored if already busy, so a click can't
    // restart mid-run. (Same guard the old start_foraging had.)
    pub fn start_skill(&mut self, i: usize, now: f64) {
        if self.active.is_none() && i < SKILLS.len() {
            self.active = Some(Active {
                skill: i,
                started_at: now,
            });
        }
    }

    // Resolve-to-now: if the active skill's duration has elapsed, finish it —
    // drop its output into the inventory and go idle. Skill-agnostic now.
    pub fn resolve(&mut self, now: f64) {
        if let Some(active) = &self.active {
            let skill = &SKILLS[active.skill];
            if now - active.started_at >= skill.duration_ms {
                *self.inventory.entry(skill.output.to_string()).or_insert(0) += 1;
                self.active = None;
            }
        }
    }

    // --- Read-only state for drawing --------------------------------------

    // Progress 0.0..1.0 of whatever is running — NOT tied to foraging.
    pub fn active_progress(&self, now: f64) -> f64 {
        match &self.active {
            None => 0.0,
            Some(active) => {
                let skill = &SKILLS[active.skill];
                ((now - active.started_at) / skill.duration_ms).min(1.0)
            }
        }
    }

    pub fn is_busy(&self) -> bool {
        self.active.is_some()
    }

    // How many of a given item we hold. JS asks per item id.
    pub fn item_count(&self, id: &str) -> u32 {
        *self.inventory.get(id).unwrap_or(&0)
    }
}
