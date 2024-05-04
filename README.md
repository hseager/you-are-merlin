# You are Merlin

A text adventure game, built with Rust. Compiles to CLI and WASM.

![CLI Screenshot](screenshots/CLI-screenshot.png)

[Click here for the WebAssembly (WASM) version with a web demo](https://github.com/hseager/you-are-merlin-www)

## Game Features

- Main quest with a final world boss, try to beat them!
- Different Themes
- Side quests
- Items
- Battles
- Recovery

### Tips

- Finding a safezone will make your journey easier!
- Try exploring to get stronger before facing the final boss
- You don't have to type out the full action, a partial match will do

## To do
- Finish adding weapon stats
- Test in WASM

### v0.2

- Better item stats (crit chance/multi/ lifegain on hit)
- Split items into types like weapons, armor, utilities with unique stats
- Inventory
- More types of encounters like puzzles and riddles etc
    - Choice encounter with reward or damage

### later

- Change rest to only partially heal
    - Gain life/potions from some encounters
- Display text rather than enemy stats
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
- Unit tests

## Building

### Development

- cargo build
- cargo run

### WASM

- wasm-pack build

### CLI Production

- cargo build -r


