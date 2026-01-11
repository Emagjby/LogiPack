# Northstar v1 — Domain + DB Foundation

## Goal
Implement the core shipment domain rules, the PostgreSQL schema (relational + Strata event store tables), and the persistence adapters that LogiCore will use.

This release is engine-first: it should be possible to build correct workflows later without refactoring boundaries.

## Non-goals
- No Auth0.
- No real UI beyond scaffolding.
- No full CRUD/API surface (only what’s needed for integration tests).

## Context Boundaries (Must Hold)
- `core-domain`: rules/types only; zero I/O.
- `core-application`: use-cases + ports/traits; orchestrates domain; no SeaORM/Axum.
- `core-data`: SeaORM entities + migrations + relational repos.
- `core-eventstore`: Strata streams/packages + append/read.
- `hub-api`: HTTP glue only.

## Locked Business Rules
- Shipment statuses (MVP): `NEW`, `ACCEPTED`, `PROCESSED`, `IN_TRANSIT`, `DELIVERED`, `CANCELLED`.
- Allowed forward progression:
  - `NEW → ACCEPTED → PROCESSED → IN_TRANSIT → DELIVERED`
- Cancellation allowed:
  - `NEW|ACCEPTED|PROCESSED|IN_TRANSIT → CANCELLED`
- Terminal:
  - `DELIVERED` and `CANCELLED`.
- Office hop policy (locked): office hop is only allowed when transitioning **to `IN_TRANSIT`**.

## Strata Storage Rules (Locked)
- Hashing uses Strata’s `hash_value` (internally BLAKE3-256) over canonical bytes.
- Event store tables:
  - `streams` (one per audited entity stream)
  - `packages` (append-only Strata blobs), linked by `prev_hash` and monotonic `seq` per stream.

## Phases

### Phase 1 — Pure Domain Model (Status Machine)
**Achievements**
- Domain types exist:
  - `ShipmentStatus`
  - transition validation (old → new)
  - terminal-state enforcement
- Domain events are represented (payload structs/enums):
  - `ShipmentCreated`
  - `StatusChanged` (must include audit-grade metadata: actor identity reference, timestamp, office context, and optional notes)

**Unit tests (Phase-backed)**
- Table-driven tests covering every allowed/disallowed transition.
- Tests proving terminal states reject any further changes.
- Tests for cancellation policy.

### Phase 2 — Relational Schema + Migrations
**Achievements**
- Migrations create at minimum these relational tables (diploma-friendly):
  - `users`, `roles`, `user_roles` (M:N)
  - `employees`, `offices`, `employee_offices` (M:N)
  - `clients`
  - `shipments` (with `current_status`, `current_office_id`)
  - `shipment_status_history` (report-friendly snapshot history)

**Unit tests (Phase-backed)**
- Unit tests that compile the SeaORM entities and repository layer modules.
- Validation unit tests in core (e.g., required fields) if implemented at this stage.

### Phase 3 — Strata Event Store Schema + Adapter
**Achievements**
- Migrations create:
  - `streams(stream_id, head_hash, kind, created_at, ...)`
  - `packages(hash, stream_id, prev_hash, scb, created_at, event_type, seq, ...)`
- Append logic:
  - calculates package hash using Strata `hash_value`
  - links package to previous head via `prev_hash`
  - increments `seq` per stream
  - updates `streams.head_hash`

**Unit tests (Phase-backed)**
- Determinism test: same payload produces same `hash_value`.
- Append rule tests:
  - first package has `prev_hash = NULL`
  - subsequent packages link to previous hash
  - `seq` increments.

## Integration Tests (Northstar-backed)
- Runs against a real Postgres instance (local or CI service):
  - apply all migrations on an empty DB
  - create a stream
  - append N packages
  - read packages back by `seq` (or by traversing hash chain) and assert integrity.

## Regression Tests (Northstar-backed)
- “Fresh DB bootstrap”: migrations succeed from zero on every test run.
- “Event chain integrity”: the stored chain cannot be broken by insert order mistakes.

## Exit Criteria
- Domain unit tests pass.
- DB migrations apply cleanly.
- Event store append/read integration test passes.

## Demo Script
1. Run migrations on a new Postgres DB.
2. Run integration test that appends and reads an event stream.
