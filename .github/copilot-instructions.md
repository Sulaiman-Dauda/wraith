---
description: "Master workflow instructions for the WRAITH project. Defines the three-agent pipeline: Code Writer → Code Reviewer → Bug Fixer. Always active."
---

# WRAITH Development Workflow

## Agent Pipeline

Every phase follows this strict pipeline:

```
┌─────────────┐     ┌───────────────┐     ┌─────────────┐
│ Code Writer │────▶│ Code Reviewer │────▶│  Bug Fixer  │
│ (Sonnet 4.6)  │     │  (Opus 4.6)     │     │  (Opus 4.6)   │
└─────────────┘     └───────────────┘     └─────────────┘
       │                    │                     │
       │                    │                     │
       ▼                    ▼                     ▼
   Writes code         Reviews for           Fixes reported
   per phase.md        correctness           issues only
                       & branding
                            │
                            ▼
                    ┌──────────────┐
                    │  Re-validate │
                    │ cargo check  │
                    │ grep audit   │
                    └──────────────┘
                            │
                            ▼
                    Phase COMPLETE
                    (wait for "next")
```

## Execution Rules

1. **One phase at a time.** Never start Phase N+1 until the user says "next."
2. **Pipeline is mandatory.** Code Writer → Code Reviewer → Bug Fixer. No skipping.
3. **Re-validate after fixes.** If Bug Fixer makes changes, run review checks again.
4. **Zero tolerance on branding.** After rename phases, `grep -ri "claw"` must return 0.
5. **Compilation is non-negotiable.** `cargo check` must pass before any phase is marked complete.

## Phase Tracking

- Current phase progress is tracked in `phase.md` (checkboxes)
- Mark tasks `[x]` as they complete
- Update `phase.md` at end of each phase

## Project Paths

| Path | Purpose |
|------|---------|
| `/home/sulaiman/Desktop/wraith/` | Project root (new repo) |
| `/home/sulaiman/Desktop/wraith/rust/` | Rust workspace |
| `/home/sulaiman/Desktop/Open Claw/claw-code-main/` | Source reference (read-only) |
| `/home/sulaiman/Desktop/Open Claw/product-plan.md` | Product specification |
| `/home/sulaiman/Desktop/Open Claw/phase.md` | Phase execution plan |
