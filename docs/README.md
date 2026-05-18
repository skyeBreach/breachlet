---
title: Breachlet Documentation
section: root
---

Reference documentation for the Breachlet project.

## Structure

Documentation is organized into sections by concern.

```text
docs/
├── README.md                        # You are here
├── definitions.md                   # Domain and architectural definitions
└── contributing/
    ├── issue_management.md           # Issue creation, workflow, and labels
    └── version_control.md            # Commit format, branching, and VCS rules
```

Each section has its own README that acts as a landing page. Root-level files cover cross-cutting concerns.

## Conventions

All documents in this directory follow these rules.

- Each document covers a single topic
- Every document uses YAML frontmatter with `title` and `section`
- Documents in `contributing/` describe how humans and tools interact with the project

## Contributing

See `contributing/issue_management.md` for how to create and classify issues.
