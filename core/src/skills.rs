// skills.rs — the CONTENT layer.
//
// A `Skill` is pure DATA: a name, how long one run takes, and what it produces.
// The engine in lib.rs runs ANY skill from this list, so "adding a skill" is
// just adding a row here — no engine code changes. This is the whole point of
// the build philosophy: front-load the system, make content cheap.

pub struct Skill {
    // Stable internal key. Never shown to the player; used in code/save data.
    pub id: &'static str,
    // The label the menu button shows.
    pub name: &'static str,
    // How long one run takes, in milliseconds.
    pub duration_ms: f64,
    // The item id this skill drops when a run finishes.
    pub output: &'static str,
    // (Later: a `inputs: &[(item, qty)]` field turns this into a full
    //  consume -> produce transform. Deliberately NOT built yet — no skill
    //  needs it. The engine would just check/spend inputs in resolve().)
}

// The master list — THIS is where content lives. Two skills to start.
pub const SKILLS: &[Skill] = &[
    Skill {
        id: "forage",
        name: "Forage",
        duration_ms: 3000.0,
        output: "berry",
    },
    Skill {
        id: "mine",
        name: "Mine",
        duration_ms: 5000.0,
        output: "ore",
    },
];
