# Northstar v0 — Monorepo Scaffold + Setup Only

## Goal
Stand up a clean monorepo skeleton that compiles and runs with minimal “hello world” behavior. This is scaffolding only: no business features, no DB schema, no Auth0.

## Non-goals
- No PostgreSQL schema or migrations.
- No SeaORM entities.
- No Strata event store tables.
- No real domain logic.
- No Auth0 integration.
- No real UI flows.

## Context Boundaries (Must Hold)
- `logipack-core/*` crates contain engine logic only and never import Hub.
- `logipack-hub/hub-api` holds HTTP concerns (Axum, auth middleware, DTO mapping).
- `logipack-hub/hub-web` is SvelteKit and talks to the API only.

## Tech Stack (Locked)
- Rust + Tokio + Axum (Hub API)
- PostgreSQL + SeaORM (introduced in v1)
- Strata packages for audit timeline (introduced in v1)
- SvelteKit (Hub Web), SSR-only data fetching (introduced in v2+)
- Ops target (introduced later): systemd services on a home server (env files stored outside git)

## Phases

### Phase 1 — Monorepo Layout + Tooling Skeleton
**Achievements**
- Repo layout follows `temp/INSTRUCTIONS.md`:
  - `logipack-core/crates/*`
  - `logipack-hub/hub-api`
  - `logipack-hub/hub-web`
- Root workspace `Cargo.toml` exists.
- Repo scripts exist (`justfile` or `makefile`) with at least:
  - `test` (runs `cargo test --workspace`)
  - `dev-api` (runs the API)
  - `dev-web` (runs the web)

**Unit tests (Phase-backed)**
- Each Rust crate contains at least one unit test (e.g. `#[test] fn smoke()`), to keep the harness alive.

### Phase 2 — Rust Workspace Compiles
**Achievements**
- Rust crates exist and compile:
  - `core-domain`
  - `core-application`
  - `core-data`
  - `core-eventstore`
  - `core-contracts`
  - `hub-api`
- `hub-api` starts and exposes `GET /health`.

**Unit tests (Phase-backed)**
- Unit test for router/handler wiring of `GET /health`.

### Phase 3 — SvelteKit Scaffolding
**Achievements**
- `logipack-hub/hub-web` exists and runs.
- One visible page renders.

**Unit tests (Phase-backed)**
- If a JS test runner is not yet selected, JS unit tests are deferred to Northstar v2.

## Integration Tests (Northstar-backed)
- Workspace smoke integration test: `cargo test --workspace` succeeds.
- Build regression: `cargo build --workspace` succeeds.

## Regression Tests (Northstar-backed)
- CI (or a local script) runs:
  - `cargo test --workspace`
  - `cargo fmt --check` (if enabled)
  - `cargo clippy` (if enabled)
  - web build (command depends on chosen package manager)
- Builds remain compatible with later systemd deployment (no hard-coded local paths).

## Exit Criteria
- Everything compiles.
- API and Web can be started locally.
- Boundaries are preserved (Hub depends on Core; Core never depends on Hub).

## Demo Script
1. Start `hub-api`, open `GET /health`.
2. Start `hub-web`, load root page.
