# Research index

Nexus-agents-style research tracking for aegis-hwsim. Every external claim in our README, architecture docs, or design decisions should trace back to an entry here.

## What lives here

| File | Purpose |
|------|---------|
| [prior-art.md](prior-art.md) | Tool-by-tool survey of adjacent projects (chipsec, fwts, LAVA, openQA, fwupd CI, ...). Why each is / isn't overlap. |
| [gotchas.md](gotchas.md) | Confirmed limitations from the community — what has broken for others trying similar approaches, with citations. |
| [audience.md](audience.md) | Target-user segments and why each would adopt. Used for first-adopter outreach planning. |

## Source-citation policy

Matches [aegis-boot](https://github.com/williamzujkowski/aegis-boot)'s `compat` DB policy: **verified outcomes only**. Each claim must link to:

- A primary source (project README, upstream documentation, official spec)
- Or a secondary source that explicitly cites the primary (Black Hat talk with cited papers, LWN article with linked mailing-list posts)
- Or a first-person observation with a reproducible repro command

"X said at a conference" without a recoverable artifact is not acceptable — someone has to be able to verify the claim without taking our word for it.

## Triage status

The initial survey (2026-04-18) was a quick pass. Re-run when:

- A new first-adopter files an issue referencing a tool we didn't catalog
- A major release lands in any of the catalogued projects (annual)
- Before cutting v1.0.0 of aegis-hwsim (gate)
