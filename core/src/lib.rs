// lib.rs — the game's LOGIC CORE, in Rust.
//
// Owns the state and the rules; knows NOTHING about pixels or buttons. The state
// lives inside `Game`; JavaScript only gets a handle and calls methods.
//
// The engine runs whatever skill is active, drawn from the data table in
// skills.rs. A skill is a TRANSFORM (consume inputs -> produce outputs), so this
// one loop covers both gathering and crafting.

use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

mod skills;
use skills::{Skill, SKILLS};

// What's currently running: which skill (an index into SKILLS) and when it began.
struct Active {
    skill: usize,
    started_at: f64,
}

#[wasm_bindgen]
pub struct Game {
    // The active skill, or None if idle.
    active: Option<Active>,
    // Everything we own: item id -> count. BTreeMap so the HUD lists items in a
    // stable, sorted order (HashMap's order would flicker frame to frame).
    inventory: BTreeMap<String, u32>,
}

// --- Private helpers (NOT exposed to JS — separate impl block so wasm-bindgen
//     doesn't try to export them) ---------------------------------------------
impl Game {
    // Do we hold enough of every input this skill needs?
    fn can_afford(&self, skill: &Skill) -> bool {
        skill
            .inputs
            .iter()
            .all(|&(id, qty)| self.item_count(id) >= qty)
    }
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            active: None,
            inventory: BTreeMap::new(),
        }
    }

    // --- Menu data ---------------------------------------------------------

    pub fn skill_count(&self) -> usize {
        SKILLS.len()
    }

    pub fn skill_name(&self, i: usize) -> String {
        SKILLS[i].name.to_string()
    }

    // Can skill `i` be started right now? False if busy or we can't afford its
    // inputs. JS uses this to enable/disable each button.
    pub fn can_start(&self, i: usize) -> bool {
        self.active.is_none() && i < SKILLS.len() && self.can_afford(&SKILLS[i])
    }

    // --- Actions -----------------------------------------------------------

    // Start skill number `i`. Ignored unless it can actually start. Inputs are
    // spent UP FRONT (we've verified we can afford them).
    pub fn start_skill(&mut self, i: usize, now: f64) {
        if !self.can_start(i) {
            return;
        }
        for &(id, qty) in SKILLS[i].inputs {
            if let Some(have) = self.inventory.get_mut(id) {
                *have -= qty;
            }
        }
        self.active = Some(Active {
            skill: i,
            started_at: now,
        });
    }

    // Resolve-to-now: if the active skill's duration has elapsed, finish it —
    // add every output to the inventory and go idle.
    pub fn resolve(&mut self, now: f64) {
        // Copy the bits we need out of the borrow on `self.active` so we're free
        // to mutate the inventory below.
        let Some((skill_index, started_at)) =
            self.active.as_ref().map(|a| (a.skill, a.started_at))
        else {
            return;
        };

        let skill = &SKILLS[skill_index];
        if now - started_at >= skill.duration_ms {
            for &(id, qty) in skill.outputs {
                *self.inventory.entry(id.to_string()).or_insert(0) += qty;
            }
            self.active = None;
        }
    }

    // --- Read-only state for drawing --------------------------------------

    // Progress 0.0..1.0 of whatever is running — NOT tied to any one skill.
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

    // How many of a given item we hold.
    pub fn item_count(&self, id: &str) -> u32 {
        *self.inventory.get(id).unwrap_or(&0)
    }

    // Every item id we currently hold (count > 0), sorted. The HUD walks this
    // and reads item_count() for each.
    pub fn inventory_ids(&self) -> Vec<String> {
        self.inventory
            .iter()
            .filter(|&(_, &qty)| qty > 0)
            .map(|(id, _)| id.clone())
            .collect()
    }
}
