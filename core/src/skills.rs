// skills.rs — the CONTENT layer.
//
// A `Skill` is pure DATA. The engine in lib.rs runs ANY skill from this list, so
// "adding a skill" is just adding a row here — no engine code changes.
//
// A skill is a TRANSFORM: it consumes `inputs` and produces `outputs`, each a
// list of (item id, quantity). This is the "inventory + transform" generalization
// from the project's build bet — gathering and crafting are the SAME shape:
//   - gather  = empty `inputs`  (Forage, Mine: make something from nothing)
//   - craft   = non-empty `inputs` (Smith: eat ore, make a bar)
// `inputs` is also what chains the passive half into the active half: mining
// drops ore (set-and-leave), smithing eats ore to make bars (log-in-to-do).

pub struct Skill {
    // Stable internal key. Never shown to the player; used in code/save data.
    pub id: &'static str,
    // The label the menu button shows.
    pub name: &'static str,
    // How long one run takes, in milliseconds.
    pub duration_ms: f64,
    // Items consumed when the run STARTS: (item id, quantity).
    pub inputs: &'static [(&'static str, u32)],
    // Items produced when the run FINISHES: (item id, quantity).
    pub outputs: &'static [(&'static str, u32)],
}

// The master list — THIS is where content lives.
pub const SKILLS: &[Skill] = &[
    Skill {
        id: "forage",
        name: "Forage",
        duration_ms: 3000.0,
        inputs: &[],
        outputs: &[("berry", 1)],
    },
    Skill {
        id: "mine",
        name: "Mine",
        duration_ms: 5000.0,
        inputs: &[],
        outputs: &[("ore", 1)],
    },
    // Demonstrates the consume side: needs 2 ore (mine it first) → 1 bar.
    // Its button stays disabled until you've mined enough.
    Skill {
        id: "smith",
        name: "Smith",
        duration_ms: 6000.0,
        inputs: &[("ore", 2)],
        outputs: &[("bar", 1)],
    },
];
