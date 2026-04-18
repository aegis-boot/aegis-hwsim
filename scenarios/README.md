# Scenarios

Scenarios are Rust test cases that drive a persona through aegis-boot's chain. Each scenario is one file, one function, one pass/fail outcome.

## Phase 1 target

- `signed-boot-ubuntu.rs` — flash the aegis-boot stick under the persona's firmware, boot it, kexec Ubuntu 24.04 live-server. Assert the full signed-chain verification path.

## Phase 2 targets

- `mok-enroll-alpine.rs` — flash, boot, observe the MOK walkthrough STEP 1/3 command line, verify `sudo mokutil --import` command accuracy against the sibling key.
- `kexec-refuses-unsigned.rs` — flash, boot, attempt kexec of an unsigned kernel, assert errno 61 + the specific diagnostic text.
- `attestation-roundtrip.rs` — flash, record the attestation manifest, boot on a persona with TPM 2.0, cross-check PCR 12 extends against the manifest.

## Adding a new scenario

Scenarios live as separate Rust files in this directory, included from the main crate via `cargo` harness config (not a `test/` directory — we need crate-level access to the runner, which `test/` doesn't give).

Each scenario signature:

```rust
/// `scenarios/my-scenario.rs`
pub fn run(persona: &Persona, stick: &Path) -> ScenarioResult {
    // QEMU invocation + assertion logic
}
```

`ScenarioResult` is pass / fail (with a diagnostic string) / skip (with a reason). No panics — all paths return `Result`-typed values.

---

Scenarios are not yet implemented. Phase 1 lands after the runner's persona loader + QEMU synthesis land.
