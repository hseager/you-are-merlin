# Blog Post
- Making a text adventure game in Rust
    - A great first project

- Getting used to the Syntax
    - How do I create a new object?
- Not great for exprimenting in code, spend time on data structure and design before starting
    - Spec things out in json first, json translates to creating Rust objects well
- This isn't the first iteration of what I built, I restructred and rewrote a lot to find the best solution


## Refactor

- GameData
    - Player
    - Locations
        - Encounters
        - Enemies

- Instances
    - State
    - Prompt
    - Actions

- Controller
    - handle_actions

1. init_game consumes Theme data rather than reference

Need to check where I'm mutating data to make sure reference lifetimes are still valid
Need to make sure references are comsumed for each loop cycle


### Gist to untangle data and mut state

https://gist.github.com/rust-play/6ff3445d37be463465d028bfcfbe69a6