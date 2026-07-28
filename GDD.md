# Game Design Document

A 2D top-down roguelite shooter inspired by Soul Knight: pixel art, procedural
dungeons, room-by-room combat, weapon variety and meta progression between runs.

This document is the source of truth for mechanics. Any pull request that
changes a mechanic must update this document in the same pull request.

## 1. MVP scope

The target is a playable vertical slice:

> one run -> 3 dungeon floors -> boss -> death or victory -> lobby with a shop

Explicitly out of scope for the MVP: multiplayer, mobile platforms, crafting,
online leaderboards.

## 2. Core loop

1. The player starts in the lobby, optionally spends coins in the shop and
   picks a character.
2. Starting a run generates a seed and the first floor.
3. A floor is a set of rooms connected by doors. Entering a combat room locks
   its doors until every enemy is dead.
4. Clearing a floor unlocks the exit to the next floor. Every third floor is a
   boss floor.
5. Death or beating the boss ends the run and returns the player to the lobby,
   keeping coins earned during the run.

## 3. Controls

The game targets desktop with a keyboard and a mouse. The layout is twin-stick
in the genre sense: moving and aiming are independent, so the player can
retreat while still shooting. No gamepad support is planned for the MVP.

| Input | Action |
| --- | --- |
| `WASD` | Move |
| Mouse position | Aim |
| Left mouse button | Fire the equipped weapon |
| `Space` | Dash |
| `E` | Interact, pick up weapon |
| `Q` | Swap between the two equipped weapons |
| `Esc` | Pause |

## 4. Player

- **Movement**: a constant top speed of 120 world units, that is 120 pixels,
  per second, with no acceleration or friction. Arcade-style instant response
  matters more than physical plausibility here. Diagonal movement is
  normalised, so holding two keys is never faster than holding one.
- **Aiming**: the player always faces the cursor, independently of the movement
  direction. Aim is stored as a unit vector so weapons never touch the cursor
  or the window directly.
- **Health**: lost permanently for the run, restored only by healing pickups.
- **Armour**: absorbs damage before health and regenerates after a few seconds
  without taking damage.
- **Energy**: consumed by firing ranged weapons and by dashing; regenerates
  slowly and from energy pickups.
- **Dash**: short burst of movement with invulnerability frames, then a
  cooldown. Melee weapons cost no energy, which keeps the player able to fight
  when out of energy.

## 5. Weapons

- Two weapon slots, one active at a time.
- Two families: melee (swing arc, no energy cost) and ranged (projectiles,
  energy cost per shot).
- Rarity tiers, in increasing order: white, green, blue, purple, orange. Rarity
  scales damage and fire rate and gates which weapons can drop on early floors.
- Stats live in RON data files, never in code, so balance can change without a
  recompile of game logic.

## 6. Enemies

Four archetypes for the MVP, all driven by a small state machine:

| Archetype | Behaviour |
| --- | --- |
| Runner | Closes distance and attacks in melee |
| Shooter | Keeps its preferred range and fires projectiles |
| Bomber | Charges the player and explodes, damaging everything nearby |
| Boss | Multi-phase, mixes projectile patterns with summoned runners |

## 7. Rooms and dungeon generation

- A floor is generated from the run seed, so the same seed always produces the
  same floor layout.
- Room types: entrance, combat, treasure, shop (optional), exit, boss.
- Rooms contain destructible props that can drop pickups.
- Generation guarantees that the exit is reachable from the entrance and that
  every room is connected.

## 8. Drops and economy

- Coins: persist after the run and are spent in the lobby shop.
- Energy and health pickups: consumed immediately, only during a run.
- Drop tables are data driven and rolled from the seeded run RNG.

## 9. Meta progression

- The lobby is the hub between runs.
- The shop offers two to three slots of permanent upgrades.
- One or two unlockable characters with different starting stats and weapons.

## 10. Technical constraints

- Engine: Bevy, pinned to `0.19` in `Cargo.toml`. Versions are never mixed.
- Architecture is ECS first: components are plain data, systems hold logic,
  resources hold global state. No inheritance-style hierarchies.
- One plugin per feature under `src/plugins/`.
- Application states: `AssetLoading -> MainMenu -> Lobby -> Run -> Paused ->
  GameOver`, driven by `NextState` with `OnEnter` / `OnExit` setup and teardown.
- Movement, damage and projectiles run in `FixedUpdate` at 60 Hz; input and
  presentation run in `Update`.
- Systems communicate through events such as `DamageDealt`, `EntityDied` and
  `ItemPickedUp` rather than calling each other directly.
- Randomness always comes from a seeded generator stored in a resource, so runs
  are deterministic and testable without a renderer. The generator is `rand`
  with `SmallRng`, approved in issue #2.
- Content and balance are authored in RON and deserialised with `serde`,
  approved in issue #9.
- Pure logic (dungeon generation, drop tables, damage calculation) lives in
  modules that do not depend on Bevy and is covered by unit tests.
