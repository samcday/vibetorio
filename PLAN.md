# Build Plan

## Milestone 0 - Deterministic Foundation (Week 1)
- [x] Document lockstep simulation contract in `docs/determinism.md`.
- [x] Add versioned lockstep packet and wire serialization helpers (`LockstepPacket`, `LockstepInput`).
- [x] Add lockstep reordering and apply helpers plus CI-tested two-peer jitter test.
- [x] Set up workspace and crate boundaries (`app`, `sim`, `content`, `save`, `tools`).
- [x] Define fixed simulation clock in `sim` (explicit ticks, no frame-time deltas in state).
- [x] Ensure input/state changes flow through `CommandQueue` and `SimulationCommand` only.
- [x] Add ordered, reproducible state hashing in `WorldGrid` and `deterministic_sim_state_hash`.
- [x] Add replay-style deterministic tests that assert same command script yields same hash.
- [x] Add a `deterministic replay` test harness that consumes saved command stream + snapshot vectors.
- [x] Publish a short lockstep CI check that validates command/snapshot replay parity.

## Milestone 1 - Core World and Commands (Weeks 2-3)
- [ ] Complete chunked infinite grid primitives and placement APIs behind deterministic IDs.
- [ ] Add command queue for all mutable player actions (place/remove/rotate/move).
- [ ] Add authoritative command validation with deterministic rejection counters.
- [ ] Add regression tests for command order, queue cap, and reject/accept parity.
- [ ] Add CLI or JSON fixture tests for command logs and world hashes.

## Milestone 2 - Resource Loop (Weeks 4-6)
- [ ] Add item stacks, world inventory abstractions, and deterministic transfer events.
- [ ] Implement chests/miners/furnaces as first machine set.
- [ ] Introduce data-driven recipes with deterministic recipe resolution.
- [ ] Add end-to-end tests proving production->storage and production->processing flows.

## Milestone 3 - Logistics v1 (Weeks 7-8)
- [ ] Implement transport belts and lane stepping logic with deterministic timing.
- [ ] Add inserter behavior and pickup/dropoff constraints.
- [ ] Extend replay hashes to cover moving-item state (or gate until state model is stable).
- [ ] Add visual/log debug tools for movement and blocked states.

## Milestone 4 - Power and Quality of Life (Weeks 9-10)
- [ ] Add power network model and throttling when no power is available.
- [ ] Add deterministic save/load version migration test in `save` crate.
- [ ] Expand command log to include authoritative state checkpoints for recovery.
- [ ] Add basic HUD showing power, throughput, and tick state.

## Milestone 5 - Stabilization and Polish (Weeks 11-12)
- [ ] Complete deterministic replay recorder/player and wire into CI as a stability gate.
- [ ] Add build checks, clippy pass, and coverage threshold in CI.
- [ ] Add first public gameplay loop goal and balancing checklist.

## Exit Criteria for each milestone
- [x] CI includes compile, deterministic replay checks, deterministic tests, and format.
- [x] At least one gameplay validation scenario exercises milestone scope.
- [x] `README` and docs updated to match behavior and command semantics.
