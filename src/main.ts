// main.ts — the BRIDGE between the Rust logic core and the screen.
//
// Reads state out of Rust and draws it. It never changes the rules. State flows
// ONE way: Rust -> screen.
//
//   - PixiJS draws the "scene" (the progress bar — now skill-agnostic).
//   - Plain DOM/HTML draws the menu, which is now DATA-DRIVEN: it asks Rust how
//     many skills exist and builds one button each. Nothing here says "Forage".

import { Application, Graphics } from "pixi.js";
import init, { Game } from "./wasm/critter_core";

(async () => {
  // 1. Load the Rust/WASM logic core.
  await init();

  // 2. Create the game. STATE lives inside Rust; `game` is just a handle.
  const game = new Game();

  // 3. PixiJS scene — the character "stage". Native resolution is a fixed 4:3
  //    (320x240); CSS scales the canvas to fit its slot (full-width on mobile,
  //    capped + centered on desktop). antialias off keeps pixel art crisp.
  //    When the critter sprite lands, set its texture scale mode to "nearest"
  //    (e.g. `sprite.texture.source.scaleMode = "nearest"`) so scaling up
  //    doesn't blur it — that's the engine's job, not the artist's.
  const app = new Application();
  await app.init({
    width: 320,
    height: 240,
    background: "#1b1f2a",
    antialias: false,
  });
  document.getElementById("pixi-container")!.appendChild(app.canvas);

  const bar = new Graphics();
  app.stage.addChild(bar);

  // Progress bar sits near the bottom of the 320x240 scene.
  const BAR_X = 30;
  const BAR_Y = 200;
  const BAR_W = 260;
  const BAR_H = 18;

  const ui = document.getElementById("ui")!;

  // 4. Build the menu from DATA. Ask Rust how many skills there are and make a
  //    button per skill. Add a skill in skills.rs and a button appears here for
  //    free — no edits to this file.
  const buttons: HTMLButtonElement[] = [];
  const skillCount = game.skill_count();
  for (let i = 0; i < skillCount; i++) {
    const button = document.createElement("button");
    button.textContent = game.skill_name(i);
    button.className = "skill-button";
    button.addEventListener("click", () => {
      // A click goes THROUGH the Rust logic. We never touch state from here.
      game.start_skill(i, performance.now());
    });
    ui.appendChild(button);
    buttons.push(button);
  }

  // 5. HUD — one count per item any skill can produce. We gather the distinct
  //    output ids once, then read their counts every frame.
  const hud = document.createElement("div");
  hud.className = "berry-count";
  ui.appendChild(hud);

  const outputs = Array.from(
    new Set(Array.from({ length: skillCount }, (_, i) => game.skill_output(i))),
  );

  // 6. Render loop — runs ~60x/second on PixiJS's clock.
  app.ticker.add(() => {
    const now = performance.now();

    // Ask Rust to advance the game to "now". A skill completes inside here.
    game.resolve(now);

    // Draw the bar from the generic progress value (any skill, not foraging).
    const progress = game.active_progress(now);
    bar.clear();
    bar.roundRect(BAR_X, BAR_Y, BAR_W, BAR_H, 6).fill("#2c3344");
    if (progress > 0) {
      bar.roundRect(BAR_X, BAR_Y, BAR_W * progress, BAR_H, 6).fill("#6fcf73");
    }

    // Update the HUD: "berry: 3   ore: 1".
    hud.textContent = outputs
      .map((id) => `${id}: ${game.item_count(id)}`)
      .join("   ");

    // Only one skill at a time: disable every button while busy.
    const busy = game.is_busy();
    for (const button of buttons) button.disabled = busy;
  });
})();
