# LogiPack

Monorepo for a logistics web application.

## Components

- **LogiPack Core**: engine (domain rules, use-cases, persistence, audit/event store).
  - Located in `logipack-core/`.
- **LogiPack Hub**: platform (Rust REST API + SvelteKit web UI).
  - Located in `logipack-hub/`.

## Dev commands

- `make test`
- `make fmt`
- `make clippy`
- `make dev-api` (available once `hub-api` exists)
- `make dev-web` (available once `hub-web` exists)

## Notes

- SvelteKit uses SSR and calls the API from the server.
- Docker Compose is intentionally not used.
