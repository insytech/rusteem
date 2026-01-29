# RustEEM Makefile
# Run all commands from the project root (rusteem/)

.PHONY: help setup deps db-start db-stop db-reset db-status db-migrate \
        dev api web build release test lint fmt check clean

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ── Setup ────────────────────────────────────────────────────────────────

setup: deps db-start db-reset ## Full setup: install deps, start DB, apply migrations
	@echo "Done. Run 'make dev' to start the API."

deps: ## Install all dependencies (Rust toolchain, Supabase CLI)
	rustup update stable
	cargo fetch
	npm install -g supabase

env: ## Create .env from .env.example
	@if [ ! -f .env ]; then \
		cp .env.example .env; \
		echo ".env created. Fill in values from 'make db-status'."; \
	else \
		echo ".env already exists."; \
	fi

# ── Database (Supabase) ─────────────────────────────────────────────────

db-start: ## Start Supabase (Postgres, Auth, Storage, Dashboard)
	npx supabase start

db-stop: ## Stop Supabase
	npx supabase stop

db-reset: ## Reset DB and apply all migrations
	npx supabase db reset

db-status: ## Show Supabase status and keys
	npx supabase status

db-migrate: ## Create a new migration (usage: make db-migrate name=add_indexes)
	@if [ -z "$(name)" ]; then \
		echo "Usage: make db-migrate name=<migration_name>"; \
		exit 1; \
	fi
	npx supabase migration new $(name)

db-diff: ## Show pending schema changes
	npx supabase db diff

# ── Development ──────────────────────────────────────────────────────────

dev: api ## Alias for 'make api'

api: ## Run the API server (debug mode)
	cargo run --bin api

web: ## Run the frontend (debug mode)
	cargo run --bin web

# ── Build ────────────────────────────────────────────────────────────────

build: ## Build all crates (debug)
	cargo build --workspace

release: ## Build all crates (release, optimized)
	cargo build --workspace --release

# ── Quality ──────────────────────────────────────────────────────────────

test: ## Run all tests
	cargo test --workspace

test-api: ## Run API tests only
	cargo test --package api

fmt: ## Format code
	cargo fmt --all

lint: ## Run clippy lints
	cargo clippy --workspace -- -D warnings -W clippy::unwrap_used

check: fmt lint test ## Run fmt + lint + tests

# ── Cleanup ──────────────────────────────────────────────────────────────

clean: ## Remove build artifacts
	cargo clean
