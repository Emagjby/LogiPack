# Northstar v2 — Shipments MVP + Dev-Secret Auth

## Goal

Deliver the first end-to-end system workflow: create shipments, transition statuses, and view an immutable Strata-backed timeline.

Auth is dev-only and deterministic (fixed secret header) to keep iteration fast.

## Non-goals

- No Auth0 yet (comes in v3).
- No full admin CRUD for clients/offices/employees yet (comes in v3).
- No reports yet (comes in v4).

## Tech + Architecture (Locked)

- API: Rust + Axum, calls LogiCore use-cases.
- Web: SvelteKit with SSR-only data fetching.
  - All API calls are made from the SvelteKit server (never directly from the browser).

## Locked Authorization Policy

- Employees may **read/list** all shipments.
- Employees may **write** only within their office scope:
  - Employee belongs to 1+ offices via `employee_offices`.
  - Create shipment: `current_office_id` must be in employee’s allowed offices.
  - Transition shipment: shipment’s current office must be in employee’s allowed offices.
  - Office hop is allowed **only** when transitioning to `IN_TRANSIT`.
    - `to_office_id` must be in employee’s allowed offices (Admin bypasses).
- Admin bypasses office scoping for write actions.

## Dev Auth (Locked)

- A fixed header must be present and valid:
  - `x-dev-secret: <known value>`
- Persona selection is explicit via a second header:
  - `x-dev-user-sub: <string>`
- The API resolves the acting identity context using `x-dev-user-sub`:
  - local `users.auth0_sub` (stored in Postgres)
  - roles from `user_roles`
  - employee + allowed offices (if applicable)

## Phases

### Phase 1 — Dev-Secret Middleware + Actor Context

**Achievements**

- API middleware rejects requests missing/incorrect `x-dev-secret`.
- API builds an actor context containing:
  - local user id or Auth0-like `sub`
  - roles (Admin/Employee)
  - employee id (if applicable)
  - allowed office IDs (from `employee_offices`)

**Unit tests (Phase-backed)**

- Missing secret → 401/403.
- Wrong secret → 401/403.
- Valid secret → success and actor context resolved.

### Phase 2 — Shipment Use-Cases + Projection Trio

**Achievements**

- Use-cases exist in `core-application`:
  - create shipment
  - change status
  - read timeline
- On create shipment:
  - create relational snapshot (`shipments`)
  - insert initial history row (`shipment_status_history`)
  - create stream and append initial Strata package
- On change status:
  - validate domain transition
  - enforce office write policy
  - write projection trio:
    1. append Strata package (immutable audit)
    2. insert history row (report-friendly)
    3. update shipment snapshot (`current_status`, and possibly `current_office_id`)
- Office hop constraint:
  - only allowed on transition to `IN_TRANSIT`

**Unit tests (Phase-backed)**

- Transition matrix: invalid transitions rejected.
- Office scope:
  - employee can’t transition shipments outside allowed offices
  - employee can’t create shipment outside allowed offices
- Office hop:
  - only allowed when new status is `IN_TRANSIT`
  - updates `shipments.current_office_id`.

### Phase 3 — API Endpoints + UI (Shipments)

**Achievements**

- API endpoints:
  - `GET /shipments` (returns all shipments)
  - `POST /shipments`
  - `GET /shipments/{id}`
  - `POST /shipments/{id}/status`
  - `GET /shipments/{id}/timeline`
- Web pages (SSR loads calling API server-side):
  - `/shipments` list + create form
  - `/shipments/[id]` detail + timeline + status actions

**Unit tests (Phase-backed)**

- Handler-level unit tests for endpoint validation and mapping.
- Timeline decoding/formatting unit tests (if any transformation is performed).

## Integration Tests (Northstar-backed)

- End-to-end against a real Postgres:
  - seed Admin user, Employee user, offices, employee_offices
  - employee creates shipment in allowed office
  - employee transitions through allowed statuses
  - timeline length matches number of transitions
  - employee forbidden write attempts outside office scope are denied

## Regression Tests (Northstar-backed)

- Transition regression:
  - ensure invalid transitions stay invalid (table-driven).
- Office hop regression:
  - only `→ IN_TRANSIT` can change `current_office_id`.
- Projection regression:
  - verify timeline append + history insert + snapshot update always stay consistent.

## Exit Criteria

- A complete shipment lifecycle demo works.
- Immutable timeline is readable and matches status history.
- Employee office restrictions are enforced for writes.

## Demo Script

1. Seed one Admin, one Employee, two offices.
2. Create shipment in Office A.
3. Transition: `NEW → ACCEPTED → PROCESSED → IN_TRANSIT` (hop to Office B) → `DELIVERED`.
4. Open timeline and verify ordered immutable entries.
