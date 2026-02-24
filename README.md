# vibe toria: Bevy Factorio-style Factory Engine Prototype

This repository is a fresh-start implementation of a deterministic, simulation-first
factory automation engine inspired by Factorio, built on top of [Bevy](https://bevyengine.org/) and its ECS.

The project is intentionally staged around a small, composable Rust workspace.

## Goals

- Build a high-performance deterministic simulation loop around a grid world.
- Keep simulation logic independent from rendering so save/load, replay, and testing stay stable.
- Use Bevy ECS for live gameplay and UI composition.
- Evolve into a modular, data-driven engine with clean boundaries between:
  - gameplay simulation
  - content definitions
  - persistence and replay
  - developer tooling

## Workspace Layout

- `crates/app` - Bevy application shell (rendering, input, camera, UI shell)
- `crates/sim` - deterministic core simulation and ECS state types
- `crates/content` - prototype/recipe/entity data definitions and loaders
- `crates/save` - serialization, versioning, save/load helpers
- `crates/tools` - CLI/dev tooling for simulation and validation

## Build and Run

```bash
cargo build
cargo run -p vibetorio-app
```

## Development Notes

- Prefer deterministic simulation steps before adding visual polish.
- Keep fixed-timestep behavior explicit; avoid frame-rate dependent simulation changes.
- Use plain data for recipes, entities, and machine tuning.
- Capture milestones and decisions in `PLAN.md`.
- See `docs/determinism.md` for lockstep protocol rules and CI sync expectations.

## License

Initial bootstrap code is licensed under `GPL-3.0-or-later` via workspace metadata.
Project-specific assets and additional dependencies should be reviewed as they are added.
