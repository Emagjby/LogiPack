# Northstar v4 — Reports + SQL Coverage

## Goal
Deliver report endpoints + UI and ensure the schema supports diploma-friendly SQL examples (JOIN/GROUP BY/subqueries) without decoding Strata blobs.

## Non-goals
- Document/diagram packaging (later).
- Post-MVP storage tiers (hot/db/cold) and GC.

## Tech + Architecture (Locked)
- Reports come from relational tables (`shipments`, `shipment_status_history`, etc.).
- Web is SvelteKit SSR-only and calls API from server.

## Locked Reporting Principle
- Reports use relational projections (fast + diploma-friendly).
- Strata remains the immutable audit source; reports do not require decoding audit blobs.

## Phases

### Phase 1 — Reports Endpoints
**Achievements**
- API endpoints implemented:
  - `GET /reports/shipments-by-status`
  - `GET /reports/shipments-by-office`
  - `GET /reports/shipments-by-client`
  - `GET /reports/shipments-by-period`
- Filtering supports basic parameters (dates, office, client, status).

**Unit tests (Phase-backed)**
- Unit tests for report query services:
  - correct grouping
  - correct filters
  - stable ordering

### Phase 2 — Reports UI
**Achievements**
- Web page `/reports`:
  - SSR load reads filter params from URL
  - server calls API
  - results render in tables

**Unit tests (Phase-backed)**
- Optional UI unit tests if a JS test runner is present.
- Otherwise, verify via integration tests.

### Phase 3 — SQL Appendix (Diploma-Friendly)
**Achievements**
- A curated set of real SQL queries exists (to be included in diploma appendix later), covering:
  - INSERT
  - SELECT
  - JOIN
  - GROUP BY
  - subquery (e.g. latest status per shipment via `shipment_status_history`)

**Unit tests (Phase-backed)**
- If SQL is stored as files, basic unit tests can ensure they remain parseable strings and reference existing table/column names.

## Integration Tests (Northstar-backed)
- Seed dataset with multiple shipments/clients/offices.
- Hit each report endpoint and assert exact aggregates.
- Validate period boundaries (inclusive/exclusive rules are explicit in API contract).

## Regression Tests (Northstar-backed)
- “Latest status” subquery regression: the last status computed by history matches `shipments.current_status`.
- Period boundary regression: shipments exactly on boundaries behave consistently.

## Exit Criteria
- Reports endpoints return correct results against seeded data.
- `/reports` renders results using SSR loads.
- SQL examples can be validated against the schema.

## Demo Script
1. Seed shipments across offices and periods.
2. Open `/reports` and demo each filter.
3. Show counts match expected.
