# AGENTS.md

Guidance for agentic coding agents working in this repository.

## Project Overview

Rust API (Axum) for Task Tracker.
Dependencies: PostgreSQL 17, Redis 7, NATS, MinIO (S3). All run via `docker-compose.yml` at the repo root.

## Build / Run / Test Commands

```sh
cargo check                        # quick compile check
cargo build                        # full build
cargo run -- serve                 # start API on http://localhost:8045
cargo test                         # run all tests (also regenerates TS types via ts-rs)
cargo test <test_name>             # run a single test
cargo sqlx migrate run             # apply database migrations (required before first build)
cargo sqlx migrate add <name>      # create a new migration
```

> **Important**: `cargo test` also exports TypeScript bindings to `ts_protocol/bindings/` via the `ts-rs` crate. The `TS_RS_EXPORT_DIR` env var is set in `api/.cargo/config.toml`.

There is no explicit clippy/rustfmt config. Run these before committing:

```sh
cargo fmt --check
cargo clippy -- -D warnings
```

## Repository Structure

```
src/
├── main.rs              # Entry point, tokio main, CLI
├── lib.rs               # Module declarations
├── config.rs            # Env-based config (Config, S3Config)
├── app_resources.rs     # Shared app state (db, nats, redis, s3, config)
├── cli.rs               # Clap CLI definition
├── db/                  # Database layer (DB* structs, sqlx queries)
├── entity/              # Domain entities (*Entity structs)
├── protocol/            # API DTOs (Request/Response, serde, ToSchema, ts-rs)
├── services/            # Business logic (AuthService, UserService, etc.)
├── router/              # Axum route handlers + custom extractors
│   ├── extractors/      # Ctx, AppJson, UserAuth, AdminAuth, etc.
│   └── workspace/       # Workspace sub-routes (avatar, invite, members, statuses, tags)
├── error/               # ApiError enum + specific error types (bad_request, unauthorized, forbidden)
├── mappings/            # Entity-to-DB / Entity-to-Protocol conversions
├── validation.rs        # Custom validation traits (ValidateBody, ValidateStringLength, etc.)
├── cache.rs             # RedisCache trait for entity caching
├── jwt.rs               # JWT create/verify helpers
├── redis.rs             # Redis connection pool (bb8)
├── nats.rs              # NATS client with retry logic
├── utils.rs             # AppTrim trait, TryIntoVec
└── swagger.rs           # OpenAPI doc generation (utoipa)
```

## Architecture & Data Flow

The API follows a layered architecture with strict separation of concerns:

```
Protocol (DTOs) ──From──> Entity (domain) ──From──> DB (persistence)
   │                        │                       │
   │  Request/Response      │  Business objects      │  sqlx row structs
   │  serde + ToSchema      │  domain logic          │  query functions
   │  ts-rs (TS export)     │  RedisCache impls      │  DB* naming prefix
```

**Data flow for a typical request:**
1. Axum route handler receives request via `AppJson<T>` extractor (auto-trims + validates)
2. Protocol type (e.g., `RegisterRequest`) converted to entity via `From` impl
3. Service layer operates on entity types, calls DB layer
4. DB layer uses `sqlx::query_as!` (compile-time checked) returning `DB*` structs
5. `DB*` structs converted to entities via `From` impl
6. Entities converted to protocol response types via `From` impl

## Code Style Guidelines

### Code Style

- **Rust edition**: 2024
- **No comments** in code unless explicitly requested
- **Import grouping**: std → external crates → crate-internal (separated by blank lines):
  ```rust
  use std::sync::Arc;

  use axum::Router;
  use serde::{Deserialize, Serialize};

  use crate::{error::ApiResult, protocol::user::UserResponse};
  ```
- **Naming**:
  - `DB` prefix for database structs (`DBUser`, `DBNewUser`, `DBUpdateUser`)
  - `Entity` suffix for domain structs (`UserEntity`, `TokensEntity`)
  - `Request`/`Response` suffix for protocol DTOs (`LoginRequest`, `LoginResponse`)
  - `snake_case` for modules, functions, files
  - `PascalCase` for types, structs, enums, traits
  - Error enums in `error/` sub-modules (e.g., `BadRequestError`, `UnauthotizedError`)
- **Derives**: Always add `Debug` and `Clone` on all structs/enums where it makes sense (protocol DTOs, entities, DB structs, services, config). Protocol types additionally include `Serialize, Deserialize, ToSchema, ts_rs::TS` with `#[ts(export)]`
- **Error handling**: Use `ApiResult<T>` (alias for `Result<T, ApiError>`). Never panic in handlers. Convert errors via `?` operator and `From` impls.
- **Validation**: Implement `ValidateBody` and `AppTrim` traits on request DTOs. Use `AppJson<T>` extractor which auto-calls `app_trim()` then `validate_body()`.
- **Custom extractors**: Add new extractors in `router/extractors/`. Follow the pattern in `app_json.rs` and `auth.rs`.
- **SQL queries**: Use `sqlx::query_as!` macro for compile-time checked queries. Place queries in `db/` modules.
- **New database table**: Create migration file in `migrations/`, add DB structs in `db/`, add entity in `entity/`, add protocol types in `protocol/`.
- **Swagger docs**: Each router module defines its own `*ApiDoc` struct with `#[derive(OpenApi)]`. Nest them in `router/mod.rs`.
- **Services**: Take `Ctx` in constructor. Access resources via `ctx.app`. Instantiate services from handlers via `ctx.<service_name>()`.

## Request Handling Conventions

### 1. Input Validation (Request DTOs)

For all create/update request DTOs (`*Request`), implement both `AppTrim` and `ValidateBody`. The `AppJson<T>` extractor auto-calls `app_trim()` then `validate_body()`, so data is validated and trimmed before reaching the handler.

### 2. Path ID Extractors

For all path parameters referencing entities (e.g. `/api/v1/{workspace_id}/tag/{tag_id}`), implement dedicated extractors in `router/extractors/`. Each extractor must:

- Verify the entity exists (including `deleted_at IS NULL` check for soft-deleted records)
- Verify parent-child relationships (e.g. tag belongs to workspace)
- Pre-load the entity data so handlers receive it directly via the extractor

Example: `WorkspaceId` extractor loads `WorkspaceEntity`, `TagId` extractor loads `TagEntity` and verifies it belongs to the workspace.

### 3. Authorization Extractors

All handlers must use specific auth/permission extractors — not just for authentication but also for authorization:

- `UserAuth` — authenticated user
- `WorkspaceMember` — user is a member of the workspace
- `WorkspaceAdmin` — user is an admin of the workspace

Choose the most restrictive extractor appropriate for the operation.

### 4. JOIN Query Naming Convention

When writing JOIN queries, name result struct fields as follows:

- Fields from the **primary table**: keep original column names (no prefix)
- Fields from **joined tables**: prefix with the table name (e.g. `workspace_name`, `user_email`)

```sql
-- Example: primary=task, joined=workspace and user
SELECT task.*, workspace.name AS workspace_name, user.email AS user_email
```

Corresponding `DB*` struct fields: `id`, `title` (from task), `workspace_name`, `user_email` (from joins).

### 5. Swagger Documentation

All route handlers must have Swagger (OpenAPI) annotations:

- `#[utoipa::path]` on every handler function with proper `request_body`, `responses`, `params`, `security` attributes
- Each router module defines its own `*ApiDoc` struct with `#[derive(OpenApi)]`
- Nest API docs in `router/mod.rs`

## Scope Boundary

This project lives inside a monorepo (`task-tracker/`), but all work must be scoped to the `api/` directory only:

- **File operations**: read, write, search, and edit only files within `api/`. Never touch sibling projects.
- **Git commands**: always filter by path using `-- api/` (e.g., `git status -- api/`, `git diff -- api/`).
- **Context**: ignore files, configs, and code outside `api/` unless explicitly asked.
- **Commit scope**: only stage and commit changes under `api/`.

## Key Conventions

- The `.env` file is committed (local dev defaults only, no secrets)
- Config is loaded from env vars with defaults in `src/config.rs`
- All API routes are nested under `/api/v1/`
- Health check endpoint at `/health`
- Swagger UI at `/api/docs`
- Passwords are SHA-256 hashed (not bcrypt — this is intentional for this project)
- Entity caching uses Redis via the `RedisCache` trait with TTL-based expiration
- Soft deletes: entities use `deleted_at` field (SQL `SET deleted_at = now()`)
