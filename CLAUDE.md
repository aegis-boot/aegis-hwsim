# aegis-hwsim — Claude Code instructions

**Project:** QEMU+OVMF+swtpm hardware-persona matrix harness for aegis-boot
**Sibling repo:** <https://github.com/williamzujkowski/aegis-boot>
**Primary tracking:** [aegis-boot#226](https://github.com/williamzujkowski/aegis-boot/issues/226)

---

## Quick orient

This repo is the **test harness** for aegis-boot's signed-chain USB-rescue flow. aegis-boot is the thing being tested; aegis-hwsim is the thing doing the testing.

Three layers:

1. **Personas** (`personas/*.yaml`) — data fixtures per shipping hardware config. DMI + Secure Boot + TPM + lockdown + quirks.
2. **Scenarios** (`scenarios/`) — Rust test cases that drive each persona through aegis-boot's chain (signed boot, MOK enrollment, kexec verification, attestation roundtrip).
3. **Runner** (`src/bin/aegis-hwsim.rs` + `src/*.rs`) — persona loader + QEMU orchestrator + coverage-grid reporter.

## Scope discipline

### In scope

- Linux-visible DMI/SMBIOS (`/sys/class/dmi/id/*`)
- Secure Boot posture via OVMF variants (MS-enrolled / custom-PK / setup-mode / disabled)
- TPM 1.2 / 2.0 via swtpm socket
- Kernel lockdown modes (none / integrity / confidentiality)
- aegis-boot-specific assertions (signed-chain boot, MOK recipe, kexec verification, attestation)

### Out of scope (permanently)

- UEFI UI automation — [LAVA documented it's unworkable](docs/research/gotchas.md#1-lava-documented-that-uefi-ui-automation-is-unworkable)
- Kernel vendor-quirk paths (thinkpad_acpi, dell-laptop, ...) — QEMU `-smbios` doesn't spoof what these check
- Hardware errata, EC bugs, broken USB controller firmware — only reproduce on metal
- UEFI firmware fuzzing — [chipsec](https://github.com/chipsec/chipsec)'s lane
- Capsule-update testing — [fwupd](https://github.com/fwupd/fwupd)'s lane

## Non-negotiables (from the aegis-boot family rulebook)

### Prime directive

```
correctness > simplicity > performance > cleverness
```

### Security constraints (from [aegis-boot#226](https://github.com/williamzujkowski/aegis-boot/issues/226) security-engineer review)

1. **Command injection** — QEMU args synthesized from YAML. Always `Command::new(...).args([...])`. Never `shell=true`, never `/bin/sh -c`.
2. **Path traversal** — YAML can reference firmware/TPM state files. Canonicalize + verify under `$AEGIS_HWSIM_ROOT/firmware/` before passing to QEMU.
3. **Resource exhaustion** — CI concurrency limits (`max-parallel: 4`), per-scenario timeouts (5 min default), swtpm cleanup in trap handlers.
4. **Test Secure Boot keys** — PK/KEK/db MUST carry `TEST_ONLY_NOT_FOR_PRODUCTION` in CN. Generated on first run. Never ship in published artifacts.

### Lint policy

Matches aegis-boot:

- `unsafe_code = "forbid"` at crate level
- `unwrap_used = "deny"`, `expect_used = "deny"` (workspace lints)
- Test modules get `#[allow(clippy::unwrap_used, clippy::expect_used)]`
- `cargo fmt` clean before every push — CI enforces

### Commit convention

Conventional commits, matching aegis-boot's enforced style:

```
feat(scope): description
fix(scope): description
refactor(scope): description
docs(scope): description
test(scope): description
chore(scope): description
```

Examples: `feat(runner): parameterized QEMU synthesis`, `docs(research): capture openQA overlap`.

### Co-authored-by

Match aegis-boot's footer convention:

```
Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
```

## Research index

Prior-art survey lives in `docs/research/`. Before adding a new integration or citing an external tool, check [docs/research/prior-art.md](docs/research/prior-art.md).

## Hardware-persona contribution policy

Mirrors aegis-boot's `compat` DB: **verified outcomes only**. A persona entering `personas/*.yaml` must cite its source via the `source` field:

- `community_report` — closed GitHub issue from a real operator who ran the full flash → boot → kexec chain
- `lvfs_catalog` — fwupd/LVFS archive URL for the firmware version
- `vendor_docs` — vendor-published spec sheet (lowest confidence)

Unsourced personas are rejected at PR review.

## Dependency notes

- **swtpm ≥ 0.8.2** — earlier versions have PCR-extend races. Document in README requirements.
- **qemu-system-x86_64 ≥ 7.2** — stable `-smbios type=4` support.
- **ovmf** — use Debian/Ubuntu packaging paths as canonical; Fedora variants mapped separately when distro support expands.
- **Rust 1.85+** — matches aegis-boot.

## Related nexus-agents memory entries

The parent session that spun up this repo has memory notes at `/home/william/.claude/projects/-home-william-git-nexus-agents/memory/`:

- `feedback_aegis_cargo_fmt.md` — CI's rustfmt is strict on method chains; run `cargo fmt` locally before every push.

## Status

**Scaffolding phase.** Subcommands currently exit 3 with "not implemented". Phase 1 target per [aegis-boot#226](https://github.com/williamzujkowski/aegis-boot/issues/226):

- [x] Persona schema + 3 starter fixtures
- [x] Research index + prior-art survey
- [ ] Runner: persona loader + `aegis-hwsim validate`
- [ ] Runner: QEMU synthesis for `signed-boot-ubuntu` scenario
- [ ] CI matrix job producing coverage grid artifact
