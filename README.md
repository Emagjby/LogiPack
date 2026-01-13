# LogiPack

Monorepo for a logistics web application (diploma project).  
Not accepting contributions right now.

## Structure

- **LogiPack Core** (`logipack-core/`)
  - Domain rules, use-cases, persistence, audit/event store.
- **LogiPack Hub** (`logipack-hub/`)
  - Platform layer: Rust REST API + SvelteKit web UI.

## Quickstart (local)

### Prereqs

- Rust toolchain (stable) + `cargo`
- Bun (for the web app)
- PostgreSQL (local install or your own container setup)

### Environment

Set `DATABASE_URL` for anything that touches the DB (migrations/tests/api), e.g.

```bash
export DATABASE_URL="postgres://USER:PASSWORD@localhost:5432/logipack"
```

### Common commands

```bash
make test    # workspace tests
make fmt     # rustfmt
make clippy  # clippy (all targets, all features)
```

If/when the hub apps exist:

```bash
make dev-api # run hub API (logipack-hub/hub-api)
make dev-web # run SvelteKit dev server (logipack-hub/hub-web)
```

## Notes / Design choices

- SvelteKit uses SSR and calls the API from the server.
- Docker Compose is intentionally not used (pick your own Postgres setup).
- `temp/` (if present locally) is a scratchpad and is never required to build/run.
