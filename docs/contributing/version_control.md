---
title: Version Control Standards
section: contributing
description: Commit message format, branching rules, and VCS conventions
---

How commits, branches, and version control are handled in the Breachlet project.

## Commit Messages

All commit messages follow the Conventional Commits specification with semantic prefixes.

### Format

```text
type(scope): subject

- Bullet one
- Bullet two

Refs #NN
```

### Subject Line

The subject line must follow these rules.

- Lowercase, no trailing period
- Imperative mood ("add feature" not "added feature")
- Keep under 72 characters
- Format: `type(scope): subject`

### Type

Matches the `type::` label taxonomy on GitLab issues.

| Type     | Use when                                |
| -------- | --------------------------------------- |
| feat     | New functionality                       |
| fix      | Bug fix                                 |
| refactor | Restructure without behavior change     |
| docs     | Documentation only                      |
| chore    | Maintenance, cleanup, tooling           |
| build    | Dependencies, build system              |

### Scope

Optional. Indicates the crate or concern affected.

- `domain` for `breachlet-domain`
- `app` for `breachlet-app`
- `infra` for `breachlet-infra`
- `core` for `breachlet-core`
- `lint` for clippy, typos, formatter configs
- `migration` for database migrations
- `docker` for container config

### Body

Required when the commit is larger than a simple change. Use bullet points to describe what changed and why.

### Footer

Reference related GitLab issues.

- `Refs #NN` when the commit relates to an issue but does not complete it
- `Closes #NN` when the commit fully resolves an issue
- `BREAKING CHANGE: summary` when the commit introduces a breaking change

### Examples

Simple change, no body needed:

```text
fix(migration): add NOT NULL to user_sessions.status
```

Larger change with body and issue reference:

```text
refactor(app): replace generics with trait objects

- Replace AppState<U> with AppState
- Use Arc<dyn UserRepository> instead of Arc<U>
- Remove generic bounds from handlers and router

Refs #8
```

Breaking change:

```text
feat(domain)!: make repo traits object-safe

- Add #[async_trait] to trait and impl blocks
- Replace impl Future returns with async fn
- Rename AuthRepository method

BREAKING CHANGE: trait signatures changed to async fn
```

## Branching

Work happens on feature branches off `main`. Do not push directly to `main` unless explicitly agreed upon.

### Branch Names

Branches should be named descriptively.

- `feat/add-unique-email-constraint`
- `fix/user-session-nullable-fields`
- `refactor/remove-generics-from-app`

## Atomic Commits

Each commit should do one thing. A commit that mixes a refactor with a feature, or a bug fix with a docs update,
is not atomic. If a change touches multiple concerns, split it into separate commits.
