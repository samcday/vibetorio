# AGENTS.md

This file captures the operating rules for all contributors and automation agents.

## Project Conventions

- Keep simulation deterministic first, visuals second.
- Write simulation-facing logic in `crates/sim` with no render-time side effects.
- Keep all entity/recipe definitions data-driven and serializable.
- Prefer small, explicit ECS systems over monolithic update functions.
- Favor clear naming and module ownership over clever abstractions.

## Rust and Bevy Standards

- Use Rust 2024 or later syntax.
- Default to Bevy ECS-first design for world state.
- `clippy` clean code is preferred; avoid warnings where practical.
- ASCII only unless a file already uses richer Unicode.

## Repository Structure

- Workspace root handles dependency and membership coordination only.
- `crates/sim` must not depend on rendering-specific Bevy crates.
- `crates/app` is allowed to depend on Bevy render/UI/audio/input crates.
- Content, save, and tools crates should stay dependency-free from `app`-only concerns.

## Testing and Validation

- Add deterministic replay or state-hash tests for each non-trivial simulation change.
- Prioritize unit tests for grid math, command validation, and save/load migrations.
- If behavior changes, capture the change in `PLAN.md` and notes in relevant PR docs.
- Do not mark milestone or checklist items complete until the validating commit has passed CI.
- CI must include coverage validation and must enforce at least 95% line coverage on code exercised by the milestone scope.

## Mode-specific Operation

- In `ELONGMODE`, operate fully self-driving and autonomous: continue implementing next logical tasks until completion.
- During `ELONGMODE`, treat `PLAN.md` as the execution plan and drive milestones to completion.
- Work strictly one feature branch at a time and gate each branch through a GitHub PR with `@codex review`.
- Use `gh` for branch management and GitHub operations; keep PRs narrow in scope.

## Contribution Notes

- Keep commits focused and small.
- Use clear module boundaries and avoid cross-crate cyclic dependencies.
- Do not introduce engine logic into markdown instructions or docs.

## Commit Conventions

- Use conventional-commit style messages.
- Keep each commit scoped to one clearly described change.
- Use short, imperative subject lines under 72 chars.
- Prefer these categories: `feat`, `fix`, `refactor`, `docs`, `test`, `build`, `chore`, `revert`.
- Suggested format:
  - `feat(sim): add fixed-tick clock state`
  - `fix(app): avoid startup panic when config missing`
  - `chore(channels): bump rust-toolchain to 1.93`

- Do not bundle unrelated refactors, rename cascades, or unrelated dependency updates in a single commit.
- Mention validation run in commit body when behavior changes.
