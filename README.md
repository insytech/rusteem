# RustEEM

Industrial Equipment, Documentation & Maintenance Management System.

Backend API built with Rust (Axum + SQLx) and Supabase as the data platform (PostgreSQL, Auth, Storage).

---

## Features

### Machine Inventory

Centralized registry of industrial equipment with location metadata (line, station, area), asset number, and active/inactive status. Supports filtering by any combination of fields and soft-delete to preserve history.

### Document Management

Versioned storage of technical documentation (3D drawings, blueprints, specifications, manuals) linked to machines. Files are stored in Supabase Storage with automatic revision control. Each re-upload of the same document type for a machine increments the revision number.

### Approval Engine

Configurable approval workflows with multiple steps and roles. Each step defines a responsible role and whether it is required. The state machine controls valid transitions:

```
draft -> pending -> approved
                 -> rejected -> draft (re-submit)
```

All actions are recorded in a complete audit history.

### Maintenance Planning

Preventive maintenance plans with configurable frequencies (hours, days, months, cycles). The system automatically calculates the next due date upon completion and allows querying upcoming or overdue plans.

---

## Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust (edition 2021) |
| HTTP Framework | Axum 0.7 |
| Database | PostgreSQL 15 (via Supabase) |
| ORM / Queries | SQLx 0.7 (compile-time checked) |
| Authentication | Supabase Auth (JWT) |
| File Storage | Supabase Storage |
| Async Runtime | Tokio |
| Middleware | Tower-http (CORS, tracing) |
| Logging | tracing + tracing-subscriber |

---

## Project Structure

```
rusteem/
├── Cargo.toml                 # Workspace root
├── .env.example               # Required environment variables
├── .rustfmt.toml              # Code formatting rules
├── supabase/
│   ├── config.toml            # Supabase CLI config
│   ├── migrations/            # SQL migrations
│   └── seed.sql               # Initial data (buckets, document types)
└── crates/
    ├── api/                   # HTTP Backend (Axum)
    │   ├── Cargo.toml
    │   ├── src/
    │   │   ├── main.rs
    │   │   ├── config.rs      # Environment variables -> AppConfig
    │   │   ├── state.rs       # AppState (pool + config)
    │   │   ├── errors.rs      # AppError enum + HTTP responses
    │   │   ├── extractors.rs  # AuthUser from JWT
    │   │   ├── middleware/     # JWT Auth, CORS
    │   │   ├── handlers/      # HTTP layer (thin)
    │   │   ├── services/      # Business logic
    │   │   └── repositories/  # SQL queries
    │   └── tests/             # Integration tests
    ├── shared/                # Shared models and DTOs
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── machine.rs
    │       ├── document.rs
    │       ├── approval.rs
    │       ├── maintenance.rs
    │       └── dto/           # Request/Response types
    └── web/                   # Frontend (Leptos)
        ├── Cargo.toml
        └── src/
```

---

## Prerequisites

- [Rust](https://rustup.rs/) (stable, >= 1.75)
- [Docker](https://docs.docker.com/get-docker/) (required by Supabase CLI)
- [Node.js](https://nodejs.org/) (>= 18, for Supabase CLI)
- [Supabase CLI](https://supabase.com/docs/guides/cli)

```bash
npm install -g supabase
```

---

## Getting Started

### 1. Clone and configure environment

```bash
cd rusteem/
make env       # Creates .env from .env.example
```

### 2. Full setup (install dependencies, start DB, apply migrations)

```bash
make setup
```

This installs Rust stable, fetches crate dependencies, installs Supabase CLI, starts all Supabase services, and applies migrations.

After setup, retrieve the generated keys and update your `.env`:

```bash
make db-status
```

Copy the `anon key`, `service_role key`, and `JWT secret` values into your `.env` file.

### 3. Start developing

```bash
make dev       # Starts the API server
make web       # Starts the frontend (in another terminal)
```

The API runs at `http://localhost:3000`, the frontend at `http://localhost:1420`.

Verify the API is running:

```bash
curl http://localhost:3000/health
# {"status":"ok","database":"connected","supabase":"configured"}
```

### Supabase services

`make db-start` automatically launches:

| Service | URL |
|---------|-----|
| PostgreSQL | `localhost:54322` |
| Supabase API | `localhost:54321` |
| Supabase Auth | `localhost:54321/auth/v1` |
| Supabase Storage | `localhost:54321/storage/v1` |
| Dashboard | `localhost:54323` |

---

## Makefile Reference

All common operations are available through `make`. Run `make help` to see the full list.

### Setup

| Command | Description |
|---------|-------------|
| `make setup` | Full setup: install deps, start DB, apply migrations |
| `make deps` | Install Rust stable, fetch crates, install Supabase CLI |
| `make env` | Create `.env` from `.env.example` |

### Database

| Command | Description |
|---------|-------------|
| `make db-start` | Start Supabase (Postgres, Auth, Storage, Dashboard) |
| `make db-stop` | Stop Supabase |
| `make db-reset` | Reset DB and apply all migrations |
| `make db-status` | Show Supabase URLs and keys |
| `make db-migrate name=<name>` | Create a new migration file |
| `make db-diff` | Show pending schema changes |

### Development

| Command | Description |
|---------|-------------|
| `make dev` | Run the API server (debug mode) |
| `make api` | Run the API server (debug mode) |
| `make web` | Run the frontend (debug mode) |

### Build

| Command | Description |
|---------|-------------|
| `make build` | Build all crates (debug) |
| `make release` | Build all crates (release, optimized) |

### Quality

| Command | Description |
|---------|-------------|
| `make test` | Run all tests |
| `make test-api` | Run API tests only |
| `make fmt` | Format code |
| `make lint` | Run clippy lints |
| `make check` | Run fmt + lint + tests |

### Cleanup

| Command | Description |
|---------|-------------|
| `make clean` | Remove build artifacts |

---

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://postgres:postgres@localhost:54322/postgres` |
| `SUPABASE_URL` | Supabase API URL | `http://localhost:54321` |
| `SUPABASE_ANON_KEY` | Supabase public key | Output of `npx supabase status` |
| `SUPABASE_SERVICE_ROLE_KEY` | Service key (backend only) | Output of `npx supabase status` |
| `JWT_SECRET` | Secret for JWT token validation | Output of `npx supabase status` |
| `ALLOWED_ORIGINS` | Allowed origins for CORS | `http://localhost:3000,http://localhost:1420` |
| `MAX_DB_CONNECTIONS` | Maximum connections in the pool | `10` |

---

## API Endpoints

### Health

| Method | Route | Description |
|--------|-------|-------------|
| GET | `/health` | Server status and DB connection check |

### Machines

| Method | Route | Auth | Description |
|--------|-------|------|-------------|
| GET | `/api/machines` | No | List machines (filters: `active`, `area`, `line`) |
| GET | `/api/machines/:id` | No | Get machine details |
| POST | `/api/machines` | Yes | Create machine |
| PUT | `/api/machines/:id` | Yes | Update machine |
| DELETE | `/api/machines/:id` | Yes | Deactivate machine (soft delete) |

### Documents

| Method | Route | Auth | Description |
|--------|-------|------|-------------|
| GET | `/api/documents` | No | List documents (filters: `machine_id`, `status`, `document_type_id`) |
| GET | `/api/documents/:id` | No | Get document details |
| POST | `/api/documents` | Yes | Create document (multipart: file + metadata) |
| PUT | `/api/documents/:id` | Yes | Update metadata |
| PATCH | `/api/documents/:id/status` | Yes | Change status |
| DELETE | `/api/documents/:id` | Yes | Delete document |

### Approvals

| Method | Route | Auth | Description |
|--------|-------|------|-------------|
| POST | `/api/documents/:id/workflows/:wid/initiate` | Yes | Start approval workflow |
| POST | `/api/approvals/:id/decide` | Yes | Submit decision (approve/reject) |
| GET | `/api/approvals/pending` | Yes | Pending approvals for authenticated user |
| GET | `/api/documents/:id/history` | No | Approval history |

### Maintenance

| Method | Route | Auth | Description |
|--------|-------|------|-------------|
| GET | `/api/machines/:id/maintenance` | No | Maintenance plans for a machine |
| GET | `/api/maintenance/upcoming` | No | Upcoming due plans (`days` query param) |
| GET | `/api/maintenance/overdue` | No | Overdue plans |
| POST | `/api/maintenance` | Yes | Create plan |
| PUT | `/api/maintenance/:id` | Yes | Update plan |
| POST | `/api/maintenance/:id/complete` | Yes | Mark completed (recalculates next_due_at) |
| DELETE | `/api/maintenance/:id` | Yes | Delete plan |

---

## Migrations

Migrations are managed with Supabase CLI and live in `supabase/migrations/`.

```bash
make db-migrate name=add_indexes   # Create a new migration
make db-reset                      # Apply all migrations (full reset)
make db-diff                       # View pending schema changes
```

---

## Tests

```bash
make test                          # All tests
make test-api                      # API tests only

# With visible logs (manual)
RUST_LOG=debug cargo test --workspace -- --nocapture
```

Integration tests require Supabase running (`make db-start`).

---

## Code Quality

```bash
make fmt                           # Format code
make lint                          # Clippy lints
make check                         # fmt + lint + tests (all at once)
```

---

## License

Proprietary software. All rights reserved.
See [LICENSE](LICENSE) for details.

Copyright (c) 2025-2026 Insytech
