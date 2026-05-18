# AGENTS.md

## Architecture

DDD-layered Rust workspace. Dependency direction matters.

- `domain` defines traits and entities. Zero framework deps (no axum, no sqlx).
  Currently has axum in Cargo.toml as a workaround for dep errors (see issue #4).
  Investigate proper fixes before adding more framework deps to this crate.
- `app` implements services and handlers using `Arc<dyn Trait>` from domain.
  Must NOT depend on `breachlet-infra` (see issue #5).
- `infra` provides concrete implementations (Pg repos) that satisfy domain traits.
  Maps DB rows to domain entities via `into_entity()`.
- `core` holds config, error types, tracing. Shared across all layers.
- `interface/server` is the binary entrypoint. Only place that wires concrete
  types to trait objects and builds AppState.
- `interface/frontend` is an empty stub. Do not remove it (issue #8 was wontfix).

## Repository Traits

All repo traits use `#[async_trait]` and return boxed futures (object-safe). Use `Arc<dyn UserRepository>`
not generics. See `domain/src/user/repo.rs` for the pattern. New repos follow the same convention.

## Config Files

All project config files use this header format (see `clippy.toml`, `typos.toml`):

```text
# ==================================================================================================================== #
# Tool - Scope
#
# Description
# ==================================================================================================================== #
# Section Name
```

Separator width is fixed. Do not change it. Section headers are named by characteristic (Policy, Complexity,
Defaults), not by file purpose.

## Docs

All docs under `docs/` use YAML frontmatter with `title` and `section`. Each directory has a `README.md` as its
landing page. Every section must have a sentence or two of lead-up text before code blocks, tables, or bullet
lists. No em dashes or double dashes in any documentation.

Lint with `markdownlint-cli2 "docs/**/*.md"` before committing docs.

## Git

Conventional commits with semantic prefixes. Full rules in `docs/contributing/version_control.md`.

- Always include `Refs #NN` or `Closes #NN` in commit body
- Do not push to `main` unless explicitly asked
- Commits must be atomic (one concern per commit)

## Tooling

- `just fmt` to run `cargo fmt` (only recipe currently)
- `cargo clippy` uses workspace `clippy.toml` + per-crate overrides
- `cargo sqlx::query` is disallowed by clippy (use `query_as` or `query_scalar`)
- `typos.toml` has `breachlet` as an allowed word

## Database

Postgres via docker-compose. Run `docker compose up -d postgres` to start. Config loaded from
`config/default.toml` with env var overrides under `BREACHLET__` prefix (double underscore separator).

Migrations run automatically on server startup via `sqlx::migrate!`.
Migration files live in `migrations/` at workspace root.

## GitLab Issues

Project at `gitlab.com/skyeBreach/breachlet`. Labels use `group::name` format.
Full taxonomy in `docs/contributing/issue_management.md`. Key rules:

- Assign to `skyeBreach` on approval
- Auto-generated issues get `source::robot` label
- Every issue needs one label from each required group (scope, type, status, source)

Open issues are tracked on GitLab. Check `glab issue list` before starting work to avoid re-introducing
known problems.

## Behavior

- Do not treat casual conversation as instructions. If the user is venting, joking, or making a passing
  observation, do not act on it unless explicitly asked to.
- Wait for explicit "go" or similar before executing. Do not run ahead.
- After each commit, stop and wait for validation before proceeding to the next one.
- Do not cargo-cult formatting from one file without checking others. Verify conventions against multiple
  examples before applying them.
- Do not add "helpful" extras (labels, files, imports) without being asked. Ask first.
- When in doubt, ask. When not in doubt, still consider asking.
