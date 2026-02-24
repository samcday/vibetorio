# Determinism and Lockstep Strategy

This document defines the deterministic simulation contract for Vibetorio.

## Core idea
- Simulation state is computed in fixed ticks, not frame time.
- Inputs are represented as deterministic commands, applied in a total order.
- Any peer can replay the same command stream and derive the same state hash.

## Lockstep contract
- The `app` crate can render, collect input, and drive UI, but only serializes domain events as commands.
- The `sim` crate owns all gameplay state transitions.
- For each tick:
  1. The app samples/receives user or network commands for that tick.
  2. `sim` applies commands in queue order.
  3. `sim` advances fixed state systems.
  4. The resulting state is hashable and checkpointable.

### Wire protocol v1
- Packets are JSON serializable and explicitly versioned.
- Core frame is `LockstepInput` with:
  - `protocol_version`
  - `tick`
  - `client_id`
  - `sequence`
  - `command`
- Client packets are wrapped in `LockstepPacket::Input` and run through `encode_lockstep_packet` / `decode_lockstep_packet`.
- For deterministic behavior in CI and replay:
  - inputs are buffered per tick,
  - sorted by `(tick, client_id, sequence)`,
  - then applied exactly once on each peer.

### Replay harness
- A replay trace is a JSON artifact with:
  - `protocol_version`
  - `start_tick`
  - `start_ticks_per_second`
  - `commands`
  - `snapshots` (expected per-tick state checkpoints)
- The simulator can execute these fixtures and verify:
  - deterministic entity/chunk counts per tick,
  - optional state hash matches when provided.
- Snapshot vectors can be used by CI as a cheap parity check before running longer scenario loops.

## Determinism rules
- No random source in simulation logic unless fed by deterministic seeds in state.
- No wall-clock time, OS entropy, or iteration-order dependent containers leaking into gameplay state.
- All command validation paths must be pure and total for the same input.
- Entity IDs, placement resolution, and occupancy checks must be stable across runs.

## Command order and replayability
- `sim` exposes a single ordered stream type: `SimulationCommand`.
- `CommandQueue` is authoritative for pending commands in a given tick.
- Queue overflow and rejection behavior must remain stable and tested:
  - command dropped counter increments on overflow
  - command rejected counter increments on validation failures
- Replay input artifacts should include the same ordered command stream.

## State hashing
- Current authoritative hash helpers:
  - `WorldGrid::deterministic_state_hash()`
  - `deterministic_sim_state_hash(clock, config, world)`
- Hashes must include all state that affects future behavior, including:
  - clock tick and tick rate
  - config values used by simulation systems
  - complete deterministic world entity ordering and transforms
- Hashes are used for:
  - regression guards in unit tests
  - optional sync checkpoints
  - replay diffing and divergence detection

## What belongs in sim (no render side effects)
- `sim` must stay free of rendering/input/audio side effects.
- `app` may:
  - run systems for rendering and UI
  - collect player intent
  - present diagnostics and visual feedback
- `sim` owns:
  - command application
  - world mutation
  - deterministic stepping and statistics

## Milestones that depend on lockstep
- Determinism scaffolding is Milestone 0 priority.
- Every new gameplay feature must include:
  - command schema/validation updates
  - unit tests for deterministic result
  - hash-aware regression coverage where behavior is stateful

## CI expectations
- CI must validate deterministic properties alongside quality gates:
  - formatting checks
  - compile/test/clippy
  - coverage threshold
  - replay-style determinism regression test pass
- CI should include a two-peer lockstep smoke test where both peers start from same seed world,
  exchange identical `LockstepPacket::Input` streams, apply them with reordering,
  and assert hash equality after each tick.

## Acceptance check
- Given two fresh worlds, two identical command scripts must produce:
  - identical end-state
  - identical `deterministic_sim_state_hash`
  - identical command accepted/rejected counts
