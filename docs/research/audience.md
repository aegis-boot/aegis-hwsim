# Target audience

Who would adopt aegis-hwsim, and what they'd get. Used for first-adopter outreach planning.

## Primary (Phase 1 outreach)

### 1. aegis-boot itself

Obvious first user. Unblocks [#51](https://github.com/williamzujkowski/aegis-boot/issues/51) (crates.io publish), [#132](https://github.com/williamzujkowski/aegis-boot/issues/132) (last-booted E2E), [#181 Phase 2](https://github.com/williamzujkowski/aegis-boot/issues/181) (atomic rotation) for the ~80% of their coverage that doesn't require physical hardware.

### 2. shim-review maintainers

<https://github.com/rhboot/shim-review>

Every distro shipping a signed shim must go through Red Hat's shim-review process. Having a reproducible pre-validation harness (run it before submitting to MS for signing) would catch regressions earlier. Worth opening a discussion once the MVP lands.

### 3. fwupd / LVFS maintainers

<https://github.com/fwupd/fwupd>

[Richard Hughes](https://github.com/hughsie) / [Mario Limonciello](https://github.com/superm1) already run QEMU+OVMF+swtpm CI for capsule updates. aegis-hwsim is a different scope (signed-chain rescue flow, not capsule update) but shares the stack. Integration or scaffolding-borrowing conversation worth having.

## Secondary (Phase 2 outreach)

### 4. Confidential-compute researchers

AMD SEV-SNP / Intel TDX / Arm CCA attestation work needs vTPM correctness testing across varied (virtual) firmware states. aegis-hwsim's attestation-roundtrip scenario with the TPM 1.2 / 2.0 / none matrix directly addresses this.

### 5. Enterprise fleet ops

Organizations with Lenovo / Dell / HP laptop fleets that need pre-deployment validation of rescue-stick flows. Less immediate adoption (enterprise procurement cycles) but long-term value. Cite by persona: "we validated aegis-boot against your exact laptop vendor + BIOS version before shipping".

### 6. Security researchers / CTF builders

Reproducible SB-bypass test beds. aegis-hwsim's persona library becomes a "what does a patched-firmware ThinkPad 2023 look like" reference. Smaller user base but high vocality.

## Tertiary (Phase 3+)

### 7. Distro SB maintainers

Ubuntu, Fedora, Debian — pre-validate shim updates before cutting. Potential integration with existing SB regression suites (openQA on SUSE's side, Ubuntu's autopkgtest on theirs).

### 8. Hardware vendors

Lenovo / Dell / Framework / HP — contribute verified DMI strings + firmware versions back upstream as community_report personas. Long tail. Probably requires someone at the vendor to care; not an early adopter.

## Non-audience (explicitly)

- Distro users who just want to boot an ISO. That's aegis-boot's audience; aegis-hwsim is a developer/researcher tool.
- Single-machine users. aegis-hwsim's value proposition is matrix coverage; running it on one persona is worse than just running aegis-boot on your actual machine.
- Windows/macOS users. Linux-only — the whole point is Linux-visible DMI + Secure Boot + TPM testing.
