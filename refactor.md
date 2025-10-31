## Refactor Plan to Reach "egui minimal app" (PRs 1–11)

### 0) Current State (as‑is)

- Single binary crate with `src/` containing `types.rs`, `rng.rs`, `state.rs`, `rules.rs`, and an `egui` app wired in `main.rs`.
- Deterministic RNG implemented (`Seeded`, `RngLike`).
- Core types (`Position`, `Direction`, `GridSize`, `Tick`).
- Game state and rules implemented: movement, wall/self collision, food spawning, score.
- Basic unit tests exist across `types`, `rng`, `state`, and `rules`.
- `main.rs` contains a very simple `eframe::App` that exposes Step/Reset and textual state; no grid render/HUD separation, no engine/UI separation, no input/time traits or update loop abstraction.

### 1) Gap Analysis vs architecture.md

- Missing engine/UI separation and traits (`Input`, `Time`) and the `systems::Loop`.
- Missing `events.rs` (domain events) – not strictly required for PR 11, but listed in architecture.
- UI code currently mixed in `main.rs`; no `input.rs` mapping UI → engine input; no `render.rs` for drawing.
- No integration tests; unit test coverage is decent but can be improved and aligned with the roadmap items.
- No CI config yet; not required for this local plan, but mentioned in roadmap.

### 2) Strategy

- Keep incremental PRs small and green; maintain deterministic behavior with `Seeded`.
- Avoid premature multi-crate split; keep a single binary crate but refactor into engine‑like modules first (traits/ports) so future split is trivial.
- Land tests alongside each change.

---

## PR‑by‑PR Plan (1–11)

### PR 1: Scaffold CI and linting (optional if local only)
- Add `cargo fmt`/`clippy` checks (CI optional if not using GitHub). 
- Ensure current code builds and tests pass with `-D warnings`.

Deliverables:
- Makefile or simple script aliases (`make check`, `make test`).

### PR 2: Core types hardening
- Review `types.rs` against architecture; keep `Tick` type even if not used yet.
- Add unit tests for basic invariants (already mostly present).

Deliverables:
- Small test additions; no functional changes.

### PR 3: RNG abstraction confirmation
- Confirm `rng::RngLike` and `Seeded` match architecture behavior. They do.
- Add one more sanity test around modulo bounds used in food spawn.

Deliverables:
- Tests only.

### PR 4: GameState init invariants
- `state::GameState::new` already spawns center snake and randomized food not on snake.
- Add/adjust tests for different grid sizes and invariants (mostly present). 

Deliverables:
- Tests; minor doc comments.

### PR 5: Movement step function
- `rules::step` already advances, checks collisions, manages growth.
- Tighten unit tests to assert non‑teleport behavior and tail pop conditions.

Deliverables:
- Tests and minor cleanup.

### PR 6: Wall collisions
- Already implemented in `rules::out_of_bounds` branch. 
- Add explicit tests per edge case (top/bottom/left/right).

Deliverables:
- Tests only.

### PR 7: Self‑collision
- Already implemented. 
- Add targeted scenarios that create a loop and validate `game_over`.

Deliverables:
- Tests only.

### PR 8: Eating & growth
- Already implemented: score increments and respawn not on snake.
- Expand tests to assert food never spawns on snake across many iterations with fixed seed.

Deliverables:
- Tests only.

### PR 9: Input trait & mock (introduce ports; no UI changes yet)
- Add `systems.rs` with `pub trait Input { fn current_dir(&self) -> Direction }` and `pub trait Time { fn tick(&mut self) -> Tick }` as per architecture.
- Add `Loop<S, T, R>` struct with `update(&mut self, g: &mut GameState)` that sets `g.snake.dir` from `input` and calls `rules::step` and `time.tick()`.
- Provide a `tests` module with a scripted input mock to validate deterministic sequences.

Deliverables:
- New `src/systems.rs` with traits and loop, plus unit tests.

### PR 10: Time trait & tick cadence
- Implement a simple `Time` mock for tests; in production UI we’ll wire it later.
- Extend tests to validate that each update advances ticks deterministically.

Deliverables:
- Tests for `Tick` increments; minor `systems.rs` updates.

### PR 11: egui minimal app – window, grid render, HUD; smoke test builds
- Introduce `input.rs` to map egui key input → `Direction`, implementing `systems::Input`.
- Introduce `render.rs` to draw:
  - Grid (cells in a fixed layout)
  - Snake body as filled rects
  - Food as a colored rect
  - HUD: score and game‑over text
- Update `main.rs`:
  - Instantiate `systems::Loop` with an egui `Input` adapter, a simple `Time` adapter, and `Seeded` RNG.
  - In `App::update`, pull input, call `loop.update(&mut game_state)`, and render via `render.rs`.
  - Keep a frame rate by calling `ctx.request_repaint()` and optionally simple time gating if desired.
- Add a smoke test: build-only or minimal run check (if headless, ensure it compiles with feature flags).

Deliverables:
- New `src/input.rs`, `src/render.rs`; update `main.rs` to use adapters and `systems::Loop`.

---

## File‑Level Changes (planned)

- `src/systems.rs` (new): `Input`, `Time`, `Loop` as per architecture sample.
- `src/input.rs` (new): egui adapter that tracks latest allowed `Direction` (disallow 180° reversals).
- `src/render.rs` (new): functions to render grid, snake, food, HUD using `egui::Painter` with cell size from available rect.
- `src/main.rs` (edit): migrate game step to `systems::Loop`; remove direct `rules::step` calls except through the loop; keep Reset; add pause toggle if convenient.
- Keep `rules.rs`, `state.rs`, `types.rs`, `rng.rs` stable except for minor pub/use adjustments.

## Testing Additions

- Add property-like tests for spawn safety across many iterations and small grids.
- Add integration test driving `Loop` with scripted directions to assert score/endings.
- Keep UI tests to smoke builds only; logic remains headless and well-tested in engine modules.

## Acceptance for PR 11

- The app opens a window and displays a grid with snake and food rendered; HUD shows score and game-over state.
- Arrow/WASD controls move the snake via the `Input` adapter.
- Game advances via the loop every frame; deterministic behavior preserved under fixed seed.
- All unit tests pass; basic smoke build test passes.

## Risks & Mitigations

- Input reversal causing instant self-collisions: adapter should prevent 180° turns in one frame.
- Coordinate→pixel mapping errors: keep a single source of truth for cell size and origin; test derived rectangles for simple cases.
- Rendering performance: keep it simple (rects only); no textures needed.

## Notes on Future Split (post‑PR 11)

- Once stable, moving `engine` modules into a dedicated crate will be trivial: `types`, `rng`, `state`, `rules`, `systems` become `engine`; `main.rs`, `input.rs`, `render.rs` remain in UI crate.



---

## Concrete Acceptance Per PR (1–11)

The following adds crisp acceptance criteria, test targets, and verification commands per PR. All criteria assume `cargo +stable` on Linux.

### PR 1: Scaffold CI and linting
- Acceptance:
  - `cargo fmt --all -- --check` passes.
  - `cargo clippy --all-targets --all-features -D warnings` passes.
  - Optional: CI config present or a local `make check` alias exists.
- Verify:
  - Run: `cargo fmt --all -- --check` then `cargo clippy --all-targets --all-features -D warnings`.
  - If using `make`: `make check`.

### PR 2: Core types hardening
- Acceptance:
  - `src/types.rs` includes `Position`, `Direction`, `GridSize`, `Tick` as per `architecture.md`.
  - Unit tests cover basic invariants (construction, equality, basic Debug/Clone usage).
- Verify: `cargo test -p <crate> types` or `cargo test` with passing suite.

### PR 3: RNG abstraction confirmation
- Acceptance:
  - `rng::RngLike` and `rng::Seeded` exist and pass determinism/sanity tests.
  - Additional test asserts modulo bounds used in food spawn are respected.
- Verify: `cargo test rng`.

### PR 4: GameState init invariants
- Acceptance:
  - `GameState::new` spawns snake at grid center and food not on snake.
  - Tests cover multiple grid sizes (odd/even) and ensure invariants (food ∉ snake, score=0, game_over=false).
- Verify: `cargo test state::new`.

### PR 5: Movement step function
- Acceptance:
  - `rules::step` advances head, conditionally pops tail, never teleports.
  - Tests validate Manhattan distance of consecutive heads is 1 and tail pop conditions.
- Verify: `cargo test rules::movement`.

### PR 6: Wall collisions
- Acceptance:
  - Hitting any boundary sets `game_over=true`.
  - Tests cover top/bottom/left/right edges on minimal grids.
- Verify: `cargo test rules::collisions::wall`.

### PR 7: Self‑collision
- Acceptance:
  - Moving into own body sets `game_over=true`.
  - Tests create a looping scenario and assert end state.
- Verify: `cargo test rules::collisions::self`.

### PR 8: Eating & growth
- Acceptance:
  - Eating food increments `score` and respawns food not on snake.
  - Tests iterate many steps with fixed seed to ensure safety (no spawn on snake).
- Verify: `cargo test rules::food`.

### PR 9: Input trait & mock
- Acceptance:
  - `src/systems.rs` defines `pub trait Input`, `pub trait Time`, and `Loop<S,T,R>` with `update` driving `rules::step`.
  - Unit tests include scripted `Input` mock for deterministic sequences.
- Verify: `cargo test systems`.

### PR 10: Time trait & tick cadence
- Acceptance:
  - A simple `Time` mock exists; `Tick` advances deterministically per update.
  - Tests assert tick increment per `Loop::update` call.
- Verify: `cargo test systems::time`.

### PR 11: egui minimal app
- Acceptance:
  - `src/input.rs` maps egui key input → `Direction`, disallowing 180° reversals in a single frame.
  - `src/render.rs` renders grid, snake, food, and HUD (score + game‑over text) using `egui::Painter`.
  - `main.rs` uses `systems::Loop` with adapters and `Seeded` RNG; window opens; frame updates request repaint.
  - Build smoke test passes; minimal run check possible locally.
- Verify:
  - `cargo run` launches window and displays grid with moving snake.
  - `cargo build --release` succeeds on stable.

---

## Test Plan Mapping (files and scopes)

- Unit tests (headless, fast):
  - `tests/rng_tests.rs`: determinism, modulo bounds.
  - `tests/types_tests.rs`: core types sanity.
  - `tests/state_init_tests.rs`: `GameState::new` invariants across grid sizes.
  - `tests/movement_tests.rs`: non‑teleport and tail pop.
  - `tests/collision_wall_tests.rs`: out‑of‑bounds → `game_over` per edge.
  - `tests/collision_self_tests.rs`: loop scenario.
  - `tests/food_spawn_tests.rs`: many iterations; food ∉ snake.
  - `tests/systems_loop_tests.rs`: `Input`/`Time` mocks, tick cadence, deterministic sequences.

- Optional integration tests:
  - `tests/integration_loop_sequences.rs`: scripted directions → score/endings with fixed seed.

Naming can be consolidated if the crate already has some tests; align with existing file names while ensuring coverage per above topics.

---

## Developer Commands and CI Pointers

- Local aliases (optional `Makefile.toml`):
  - `make check`: `cargo fmt --all -- --check && cargo clippy --all-targets --all-features -D warnings`
  - `make test`: `cargo test --all`
  - `make cov`: integrate `llvm-cov` or `grcov` later as in `architecture.md`.
  - `make bench`: optional `criterion` benches for `rules::step`.

- CI (when enabled):
  - Matrix stable + nightly.
  - Steps: checkout → cache cargo → fmt/clippy → build → test → upload coverage.

---

## Risks, Rollbacks, and Guardrails

- Input reversal → instant self‑collision:
  - Mitigation: adapter disallows 180° reversal within the same frame; respect last settled direction.

- Coordinate→pixel mapping errors:
  - Mitigation: single function to derive cell size/origin in `render.rs`; keep constants centralized; consider a unit test validating mapping for a tiny grid (no pixel diffs).

- RNG loops on nearly full grids:
  - Mitigation: tests cover edge cases; consider bounded attempts with fallback in future features.

- Keep PRs ≤300 LOC delta; each PR must stay green (fmt, clippy, tests) before proceeding.

---

## Definition of Done for the Minimal App (end of PR 11)

- Engine‑like modules in place (`types`, `rng`, `state`, `rules`, `systems`).
- UI adapters present (`input.rs`, `render.rs`), and `main.rs` wired to `systems::Loop` with `Seeded` RNG.
- Arrow/WASD controls move snake; reversal prevention active.
- Grid, snake, food, and HUD render; game‑over state displays.
- Reproducible behavior under fixed seed in tests; unit tests pass with high coverage of `rules`.
- `cargo fmt`/`clippy -D warnings` clean; `cargo build --release` succeeds on stable.

---

## Next Steps (beyond PR 11)

- Optional domain events module (`events.rs`) to support richer telemetry and replay.
- Introduce settings/persistence under feature flags as in `architecture.md`.
- Prepare for workspace split into `engine` and UI crates once stable.
