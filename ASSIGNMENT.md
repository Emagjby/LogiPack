# ASSIGNMENT.md

## Project

**Diploma Project (Applied Programmer)**  
**Topic:** Web application for a logistics company (information system)  
**System name (product line):**

- **LogiPack Hub**: the platform (Web UI + REST API)
- **LogiCore**: the engine (domain logic + data access layer + auditing)

This repo is built primarily for _me_ (developer notes + execution plan), but it maps 1:1 to the official diploma requirements.

---

## 1) Problem domain overview

Logistics companies manage shipments moving through offices and processes: intake, processing, transport, delivery. Typical operational problems:

- tracking shipment status and current location (office)
- managing clients, offices, employees
- ensuring role-based access (admin vs employee)
- generating reports (by status, client, period, office)
- maintaining auditability (who changed what, when, where)

The goal of this project is to deliver a working web system that covers the core workflows and provides clear technical documentation (architecture, ER diagram, UML, SQL queries, UI screenshots).

---

## 2) Goals

### Primary goals (MVP)

1. Manage **clients** (create/edit/list).
2. Manage **offices** and **employees** (create/edit/list).
3. Manage **shipments**:
   - create shipment
   - process shipment through status lifecycle
   - track current status and current office
4. Implement **roles and permissions**:
   - Admin
   - Employee
5. Provide **reports**:
   - by status
   - by client
   - by period
   - by office
6. Implement **immutable audit timeline** for shipment state transitions using Strata packages.

### Secondary goals (post-MVP, future work)

- Introduce **3-layer data access** for immutable timeline lookups:
  - Hot Storage (in-memory cache of decoded Strata by hash)
  - DB (PostgreSQL event store + relational model)
  - Cold Storage (archival for older immutable packages)
- Add smart **garbage collection** and scalability mechanisms
- Mobile version
- Map integration (shipment location visualization)
- Email notifications
- DB optimization

---

## 3) Tech stack

### Backend

- **Rust** REST API
- **SeaORM** (ORM)
- **PostgreSQL** database
- Architecture style: layered (Domain/Application/Data) and MVC mapping

### Frontend

- **SvelteKit**
- UI fetches the Rust API (client-side fetch)

### Authentication

- **Auth0** for identity/auth boilerplate elimination
- Local DB stores application roles (Admin/Employee) linked to Auth0 identity

### Tooling / process

- GitHub (source control)
- UML diagrams (use-case, component/architecture, ER)
- SQL queries (INSERT/SELECT/JOIN/GROUP BY/subqueries)
- **No Docker Compose**: local dev + CI + hosting setup from scratch

---

## 4) Status lifecycle (Shipment State Machine)

Supported statuses (MVP):

- `NEW`
- `ACCEPTED`
- `PROCESSED`
- `IN_TRANSIT`
- `DELIVERED`
- `CANCELLED`

Rules (high-level):

- forward progression allowed: `NEW → ACCEPTED → PROCESSED → IN_TRANSIT → DELIVERED`
- cancellation allowed from non-final states (policy choice): `NEW|ACCEPTED|PROCESSED|IN_TRANSIT → CANCELLED`
- `DELIVERED` and `CANCELLED` are terminal states

Each transition records immutable metadata (audit):

- who performed the change (employee)
- when (timestamp)
- where (office context)
- optional “from office” → “to office” transfer data  
  Example: Office A → Office B → … → Office Y (yes, the math joke is canon).

---

## 5) Persistence model (hybrid)

### 5.1 Relational model (mutable operational data)

Used for:

- clients, offices, employees
- current shipment view (current status, current office, assigned employee, etc.)
- reporting queries (fast SQL JOIN/GROUP BY)

### 5.2 Strata event store (immutable audit timeline)

Used for:

- immutable shipment events and state transition metadata
- verifying history and traceability

**Important:** We do _not_ use Strata for everything. Only for immutable/audit-grade facts. “Cosmetic” and editable entities remain relational.

---

## 6) Database design requirements mapping

Diploma requirement: **minimum 6 tables + at least two M:N relations**.

### 6.1 Required M:N relations

1. Users ↔ Roles (Admin/Employee; scalable for future roles)

- `users`
- `roles`
- `user_roles` (M:N)

2. Employees ↔ Offices (employees can belong to multiple offices; offices have multiple employees)

- `employees`
- `offices`
- `employee_offices` (M:N)

### 6.2 Core relational tables (suggested)

- `users` (linked to Auth0 subject)
- `roles`
- `user_roles` (M:N)
- `clients`
- `offices`
- `employees`
- `employee_offices` (M:N)
- `shipments`
- `shipment_status_history` (optional but recommended for reporting + UX)

### 6.3 Strata event store tables

- `streams` (one stream per audited entity, e.g., a shipment stream)
- `packages` (immutable Strata SCB blobs, chained by hash)

---

## 7) ER design (conceptual)

### 7.1 Shipment current view (relational)

`shipments` stores operational fields:

- sender client, receiver client
- weight/type/price
- `current_status`
- `current_office_id`
- timestamps

### 7.2 Status history (relational, report-friendly)

`shipment_status_history` stores:

- `shipment_id`
- `status`
- `changed_at`
- `office_id`
- `changed_by_employee_id`

This table allows simple SQL queries for the diploma (JOIN/GROUP BY/subqueries) without decoding SCB blobs.

### 7.3 Immutable timeline (Strata packages)

Each shipment has a `stream_id`. Each transition appends a new package (SCB) and moves `streams.head_hash`.
The relational tables are updated as a projection/snapshot (current view).

---

## 8) MVC mapping (as required)

The diploma uses MVC terminology in a web app context:

- **Model**
  - SeaORM models for relational tables (`clients`, `shipments`, `offices`, etc.)
  - SeaORM models for event store tables (`streams`, `packages`)
- **Controller**
  - Rust API handlers (REST endpoints)
- **View**
  - SvelteKit pages/components and UI state

---

## 9) System architecture

### 9.1 Components

- **LogiPack Hub API (Rust)**
  - REST endpoints
  - authorization checks (roles)
  - orchestration of use-cases
  - writes to PostgreSQL (relational + event store)
- **LogiPack Hub Web (SvelteKit)**
  - UI for Admin/Employee workflows
  - fetches API endpoints
- **LogiCore (engine)**
  - domain rules (status transitions)
  - application use-cases (create shipment, change status, generate reports)
  - data access layer (SeaORM repositories)
  - audit/event append logic for Strata packages

### 9.2 Data flow (status update)

1. UI requests: “update shipment status to PROCESSED”
2. API checks role and validates transition
3. API appends Strata package (immutable event) to shipment stream
4. API updates relational snapshot:
   - `shipments.current_status`
   - `shipments.current_office_id` (if transfer happened)
   - inserts row into `shipment_status_history`

Result: fast UI + strong audit trail.

---

## 10) Authentication and authorization

### Auth0

- Auth0 provides identity; the API validates tokens.
- Auth0 `sub` is mapped to internal `users.auth0_sub`.

### Roles (scalable)

- MVP roles: `ADMIN`, `EMPLOYEE`
- DB-based role assignment via `user_roles`
- Easy to extend later (e.g., CLIENT, MANAGER)

Authorization strategy:

- API endpoints require certain roles
- reports and administration restricted to Admin (policy choice)

---

## 11) Functional requirements (MVP)

### 11.1 Clients

- Create client
- Edit client
- List/search clients

### 11.2 Offices

- Create/edit/list offices

### 11.3 Employees

- Create/edit/list employees
- Assign employee to offices (M:N)

### 11.4 Shipments

- Create shipment (NEW)
- Move through statuses
- Track current status and current office
- Cancel shipment (if allowed)

### 11.5 Reports

- Shipments by status
- Shipments by client
- Shipments by period
- Shipments by office

---

## 12) UI pages (SvelteKit)

MVP pages (suggested):

- `/login` (Auth0 flow)
- `/dashboard` (overview + quick stats)
- `/clients` (list + create/edit)
- `/offices` (list + create/edit)
- `/employees` (list + create/edit + office assignment)
- `/shipments` (list + create)
- `/shipments/[id]` (details + timeline + status actions)
- `/reports` (filters: status/client/period/office)

Screenshots of key pages will be included in the diploma documentation.

---

## 13) API endpoints (REST, suggested)

### Auth / user

- `GET /me` (current user + roles)

### Clients

- `GET /clients`
- `POST /clients`
- `GET /clients/{id}`
- `PUT /clients/{id}`

### Offices

- `GET /offices`
- `POST /offices`
- `PUT /offices/{id}`

### Employees

- `GET /employees`
- `POST /employees`
- `PUT /employees/{id}`
- `PUT /employees/{id}/offices` (assign offices)

### Shipments

- `GET /shipments`
- `POST /shipments`
- `GET /shipments/{id}`
- `POST /shipments/{id}/status` (transition + metadata)
- `GET /shipments/{id}/timeline` (read Strata audit timeline)

### Reports

- `GET /reports/shipments-by-status`
- `GET /reports/shipments-by-office`
- `GET /reports/shipments-by-client`
- `GET /reports/shipments-by-period`

---

## 14) SQL requirements (examples to include in diploma appendix)

### INSERT (create shipment)

- Insert into `shipments`
- Insert initial status row into `shipment_status_history`

### SELECT + JOIN (shipments with current office)

- Join `shipments` ↔ `offices`
- Join sender/receiver clients

### GROUP BY (shipments count by status)

- Group by `shipments.current_status` or by history rows

### Subquery (shipments whose last status is X)

- Using `shipment_status_history` with a subquery selecting latest row per shipment

(Exact SQL queries will be written as part of the “SQL Queries” section and validated against the final schema.)

---

## 15) Testing strategy

### Unit tests

- Status transition rules (state machine)
- Validation logic (weight/price, required fields)

### Integration tests

- Repository operations with Postgres (SeaORM)
- Shipment creation + status update workflow

### API tests (minimal)

- Role-restricted endpoints
- Shipment status change endpoint
- Reports endpoints

---

## 16) Delivery / demo plan (no Docker Compose)

Local dev:

- Postgres installed locally (or manual run, but not compose)
- API and Web started via repo scripts (`just`, `make`, or shell scripts)

CI (GitHub Actions):

- format + lint + tests
- build API + build web
- optional: spin Postgres service in CI (native CI service, not compose)

Deployment (from scratch):

- systemd services for API and web (or reverse proxy setup)
- environment configuration documented in `ops/`

---

## 17) Milestones

1. Repo scaffolding + DB schema (ER) + migrations
2. Auth0 integration + roles mapping
3. CRUD: clients/offices/employees
4. Shipments: create + list + details
5. Status transitions + relational history + Strata timeline append
6. Reports + required SQL queries
7. UI polish + screenshots
8. Final documentation: architecture + UML + conclusion + references

---

## 18) Documentation artifacts to produce (diploma-friendly)

- Introduction: logistics domain + typical software solutions
- Chapter 1 (Theory):
  - logistics processes
  - client-server architecture
  - relational DB model (keys/relations)
  - tech overview (Rust, Postgres, SeaORM, MVC, SvelteKit)
  - software design principles + diagrams
- Chapter 2 (Implementation):
  - system architecture
  - ER diagram (6+ tables, 2 M:N)
  - models/controllers/views mapping
  - features walkthrough
  - UI screenshots
  - SQL queries
- Conclusion: what problems it solves + improvements
- References: Rust/SeaORM/Postgres/SvelteKit/Auth0 docs, articles, etc.

---

## 19) Notes (developer intent)

This system deliberately mixes:

- **relational operational data** for usability and reporting
- **immutable Strata audit timeline** for traceability and correctness

Post-MVP, the immutable timeline can scale via hot/db/cold layers and garbage collection, but MVP focuses on correctness and complete diploma coverage.

(And yes: Office A → Office B → Office Y is officially blessed.)
