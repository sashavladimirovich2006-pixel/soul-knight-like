# Roadmap

Goal: a playable vertical slice, described in `GDD.md`. Anything that does not
serve the slice belongs in the backlog, not in the code base.

## Milestone 0 - foundation (in progress)

- [x] Pin the Bevy version and set up the project skeleton
- [x] `AppState` state machine and a fixed 60 Hz gameplay timestep
- [x] CI running `cargo fmt`, `cargo clippy -D warnings` and `cargo test`
- [ ] Seeded run RNG resource
- [ ] Nearest-neighbour texture sampling for crisp pixel art
- [ ] Loading of RON data files for balance and content

## Milestone 1 - the player moves and shoots

- [ ] Twin-stick movement in `FixedUpdate`, aiming in `Update`
- [ ] Health, armour with regeneration, energy
- [ ] Dash with invulnerability frames and cooldown
- [ ] First ranged weapon with an energy cost per shot
- [ ] First melee weapon
- [ ] Damage events and death events

## Milestone 2 - a room that fights back

- [ ] Room as a data structure with doors
- [ ] Doors lock on entering a combat room and unlock when it is cleared
- [ ] Runner and shooter enemies with a simple state machine
- [ ] Bomber enemy
- [ ] Destructible props
- [ ] Coin, health and energy pickups with data driven drop tables

## Milestone 3 - a floor and a run

- [ ] Seeded floor generation with connectivity guarantees, unit tested
- [ ] Transition between floors
- [ ] Boss floor every third floor, with one multi-phase boss
- [ ] Run ends on death or victory and reports the result

## Milestone 4 - meta progression

- [ ] Lobby state with its own scene
- [ ] Shop with two to three permanent upgrade slots
- [ ] Persisting coins between runs
- [ ] One unlockable character

## Backlog, deliberately not in the MVP

- Multiplayer, mobile platforms, crafting, online leaderboards
- Weapon modifiers and affixes beyond plain rarity tiers
- Audio design beyond placeholder sounds
- Controller support
- Build caching in CI, which currently rebuilds the engine from scratch
