# Build Plan

## Milestone 0 - Foundation (Week 1)
- [x] Set up workspace and crate boundaries (`app`, `sim`, `content`, `save`, `tools`).
- [x] Add deterministic tick loop in `sim`.
- [x] Add minimal Bevy app bootstrap with empty scene and startup/update systems.
- [x] Seed an empty simulation world resource and a per-tick world-step stub.
- [x] Add smoke tests for deterministic tick and empty-world progression.

Milestone 0 exit condition currently met: you can build and start the app with an
empty simulation world state.

## Milestone 1 - Core World and Commands (Weeks 2-3)
- Implement chunked infinite grid primitives and placement APIs.
- Add command queue for player actions (place/remove/rotate/move entities).
- Add authoritative command validation in simulation.
- Add debug rendering overlay for tile grid and command latency.

## Milestone 2 - Resource Loop (Weeks 4-6)
- Add item stacks, world inventory abstractions, and item transfer events.
- Implement chests/miners/furnaces as first machine set.
- Introduce deterministic production recipes from data files.
- Add end-to-end test that proves miner->chest and miner->furnace flows.

## Milestone 3 - Logistics v1 (Weeks 7-8)
- Implement transport belts and lane stepping logic.
- Add inserter behavior and pickup/dropoff constraints.
- Add path-debug tools to visualize movement and blocked states.

## Milestone 4 - Power and Quality of Life (Weeks 9-10)
- Add power network model and throttling when no power is available.
- Add save/load version migration test in `save` crate.
- Add basic HUD showing power, throughput, and ticks.

## Milestone 5 - Stabilization and Polish (Weeks 11-12)
- Add replay recording + deterministic replay verification.
- Add build checks, clippy pass, and CI skeleton.
- Add first public gameplay loop goal and balancing checklist.

## Exit Criteria for each milestone
- CI checks for crate compile, deterministic tests, and format.
- At least one gameplay validation scenario that exercises the milestone scope.
- README and docs updated to match new behavior and command usage.
