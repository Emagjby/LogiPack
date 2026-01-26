# Northstar v3 — Auth0 + Roles + Admin CRUD

## Goal

Make the system production-shaped by replacing dev auth with Auth0 JWT validation and implementing role-based + office-based authorization. Add Admin CRUD for clients, offices, employees, and employee↔office assignment.

## Non-goals

- Reports (v4).
- Diploma diagrams/screenshots (later).

## Tech + Architecture (Locked)

- API: Rust + Axum
- Auth: Auth0 JWT validation (RS256 via JWKS; dev/test may use local JWKS to avoid network dependence)
- Roles: stored in Postgres and mapped by Auth0 `sub`
- Web: SvelteKit SSR-only; server calls API (browser never calls API directly)

## Locked Authorization Policy

- Admin bypasses office scoping.
- Employees may read/list all shipments.
- Employees may write only within allowed offices:
  - Create shipment only in an allowed office.
  - Transition shipment only if shipment’s `current_office_id` is allowed.
  - Office hop only on transition to `IN_TRANSIT`, and `to_office_id` must be allowed.

## Phases

### Phase 1 — Auth0 JWT Validation + Local User Mapping

**Achievements**

- API validates JWT (issuer/audience/signature).
- Extracts `sub` and maps to local `users.auth0_sub`.
- Loads roles from `user_roles`.

**Unit tests (Phase-backed)**

- Token validation tests using test keys.
- Role resolution tests from seeded in-memory/mock repos.

### Phase 2 — Authorization (Roles + Office Scope)

**Achievements**

- Centralized policy layer exists (single source of truth).
- Endpoints enforce required roles.
- Office scoping is enforced for employee write actions.

**Unit tests (Phase-backed)**

- Policy matrix tests:
  - (Admin/Employee) × (office membership) × (action)
  - ensures “read-all shipments” stays read-only.

### Phase 3 — Admin CRUD (API + Web)

**Achievements**

- Admin API endpoints:
  - clients CRUD (create/edit/list)
  - offices CRUD
  - employees CRUD
  - assign employee to offices (M:N)
- Web pages (SSR loads calling API server-side):
  - `/clients`
  - `/offices`
  - `/employees`
  - employee↔office assignment UI

**Unit tests (Phase-backed)**

- CRUD validation unit tests (DTO-level).
- Assignment rule unit tests (idempotency, duplicates).

## Integration Tests (Northstar-backed)

- Authenticated integration tests using test JWTs:
  - Admin can perform CRUD.
  - Employee cannot perform admin CRUD.
  - Employee still can perform shipment actions as office rules allow.

## Regression Tests (Northstar-backed)

- Role regression:
  - endpoints never become public by accident.
- Policy regression:
  - employee cannot bypass office constraints.

## Exit Criteria

- Auth0 auth works end-to-end (or with test JWKS in dev).
- Admin management screens work.
- Employee permissions are correct.

## Demo Script

1. Login as Admin → manage clients/offices/employees.
2. Login as Employee → create/transition shipment within allowed office.
3. Show denial when attempting admin CRUD as Employee.
