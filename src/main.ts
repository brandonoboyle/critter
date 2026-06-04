// main.ts — the BRIDGE between the Rust logic core and the screen.
//
// This is the only file that knows about both sides. Its job: read state out of
// Rust and draw it. It never changes the game's rules — it just asks Rust what's
// true and paints that. State flows ONE way: Rust -> screen.
//
// Split of responsibilities here:
//   - PixiJS draws the "scene" (the progress bar).
//   - Plain DOM/HTML draws the menu (the Forage button + berry count) — the part
//     you already know well.

import { Application, Graphics } from "pixi.js";
// This import is our Rust crate, compiled to WASM by wasm-pack.
//   `init` loads the .wasm file (async — it must finish before we use anything).
//   `Game` is our Rust struct, usable here as a JavaScript class.
import init, { Game } from "./wasm/critter_core";

(async () => {
  // 1. Load the Rust/WASM logic core.
  await init();

  // 2. Create the game. The STATE lives inside Rust now; `game` is just a handle.
  //    JS can call its methods but can't reach in and edit the state directly.
  const game = new Game();

  // 3. PixiJS scene — just a progress bar for now.
  const app = new Application();
  await app.init({ width: 360, height: 160, background: "#1b1f2a" });
  document.getElementById("pixi-container")!.appendChild(app.canvas);

  const bar = new Graphics();
  app.stage.addChild(bar);

  const BAR_X = 30;
  const BAR_Y = 70;
  const BAR_W = 300;
  const BAR_H = 24;

  // 4. DOM menu — the button and the berry count.
  const button = document.createElement("button");
  button.textContent = "Forage";
  button.className = "skill-button";
  button.addEventListener("click", () => {
    // A click goes THROUGH the Rust logic. We never touch pixels from here.
    game.start_foraging(performance.now());
  });
  document.getElementById("ui")!.appendChild(button);

  const berryLabel = document.createElement("div");
  berryLabel.className = "berry-count";
  document.getElementById("ui")!.appendChild(berryLabel);

  // 5. Render loop — runs ~60x/second on PixiJS's clock.
  app.ticker.add(() => {
    const now = performance.now();

    // Ask Rust to advance the game to "now". A forage completes inside here.
    game.resolve(now);

    // Read values back out of Rust. Only plain numbers/bools cross the boundary.
    const progress = game.foraging_progress(now);

    // Redraw the bar: a grey track, then a green fill scaled by progress.
    bar.clear();
    bar.roundRect(BAR_X, BAR_Y, BAR_W, BAR_H, 6).fill("#2c3344");
    if (progress > 0) {
      bar.roundRect(BAR_X, BAR_Y, BAR_W * progress, BAR_H, 6).fill("#6fcf73");
    }

    // Update the DOM HUD.
    berryLabel.textContent = `Berries: ${game.berries()}`;
    button.disabled = game.is_foraging(); // can't forage while foraging
  });
})();
