# You are Merlin

A text adventure game, built with Rust. Compiles to CLI and WASM.

![CLI Screenshot](screenshots/CLI-screenshot.png)

[Click here for the WebAssembly (WASM) version with a web demo](https://github.com/hseager/you-are-merlin-www)

## Game Features

- Main quest with a final world boss, try to beat them!
- Themes like Zelda, Star Wars and Fallout
- Side quests
- Random loot generation and rewards
- Battles
- Recovery
- Inventory

### Tips

- Finding a safezone will make your journey easier!
- Try exploring to get stronger before facing the final boss
- You don't have to type out the full action, a partial match will do

## To do

### v0.2
- Adjust items gen
    - Can't get common items at the moment
- Test in WASM
- Add more weapon data for each theme so we don't run out
- Balance changes
    - Add stats to enemies
- Display text rather than enemy stats

- More types of encounters like puzzles and riddles etc
    - Choice encounter with reward or damage

### later
- Manage during battles
- Change rest to only partially heal
    - Gain life/potions from some encounters
- Usable items
    - Use items/potions during a battle loop
- Change battle loop to be interactive
    - Parry just in time of enemies attack to get bonus damage etc
- Rewards (gold) and trade
- Boss quote
- chatGPT API
- Chance for Run to fail
- Mini bosses for dungeons
- Dungeon difficulty, more battles = better reward
- Require key for main boss
- Better visuals for life/attack

### Refactor

- Stop cloning everything... try to reference locations etc

## Building

### Development

- cargo build
- cargo run

### WASM

- wasm-pack build

### CLI Production

- cargo build -r


