# Prior-art survey

Initial capture: 2026-04-18. Sources: nexus-agents `research_discover` + direct web search (Google + vendor project pages). Individual tool links verified at capture time; re-verify before citing in release notes.

## Methodology

Questions the survey answered:

1. Does a persona-matrix QEMU+OVMF+swtpm harness already exist?
2. If partial overlap exists, what differentiates aegis-hwsim?
3. What industrial-scale QEMU+OVMF+swtpm CI exists we can learn from?
4. What failure modes have others hit?

Result: no direct replacement; two near-overlaps (fwupd's capsule CI, SUSE openQA's SB regression tests). aegis-hwsim's differentiation is **persona-driven matrix (per laptop-vendor) keyed on real shipping hardware DMI**, combined with **aegis-boot-specific assertions** (signed-chain USB rescue, MOK recipe accuracy, kexec verification, attestation roundtrip).

## Tool catalog

### Direct references (reused, linked from README)

| Tool | URL | Scope | Relationship |
|------|-----|-------|--------------|
| **chipsec** | <https://github.com/chipsec/chipsec> | Intel-authored UEFI/platform security scanner; audits real platforms (SPI/BIOS/SMM/SB config) | Complementary — live-system audit, not emulated matrix. Use as reference for what SB-config checks look like. |
| **fwts** | <https://github.com/fwts/fwts> | Canonical's Firmware Test Suite — ACPI/UEFI/SMBIOS conformance | Complementary — firmware correctness; we test OS-level SB-chain flow. |
| **edk2 SCT / UEFI SCT** | <https://github.com/tianocore/edk2-test> | UEFI spec conformance tests for firmware implementers | Orthogonal — validates firmware implementations, not OS flows. |
| **KernelCI** | <https://kernelci.org> | Upstream kernel CI on real boards + QEMU | Adjacent — kernel regression, not SB/TPM scenario matrix. |
| **LAVA (Linaro)** | <https://docs.lavasoftware.org> | Deployment orchestrator for lab boards + QEMU; YAML test definitions | **Integration target**, not competitor. aegis-hwsim could emit LAVA job definitions as an optional export. |
| **labgrid** | <https://github.com/labgrid-project/labgrid> | pytest-style HW-in-loop harness | Same pattern, real-hw focused. aegis-hwsim could ship a labgrid pytest collector. |
| **OSBuild** | <https://www.osbuild.org> | Image builder with boot-test plugin via QEMU | None — builds images, doesn't validate SB chain across personas. |
| **fwupd QEMU+OVMF+swtpm CI** | <https://github.com/fwupd/fwupd> | Industrial capsule-update CI using the same stack | **Reference** — study scaffolding patterns, don't rewrite. |
| **openQA (SUSE)** | <https://open.qa> | VM-based distro regression; exercises shim/grub SB paths per distro | Adjacent — per-distro coverage, not per-laptop-vendor. |
| **sbctl** | <https://github.com/Foxboron/sbctl> | SB key management tooling | Complementary. |
| **shim-review** | <https://github.com/rhboot/shim-review> | Upstream shim review process | Process, not code. Potential first-adopter audience. |
| **puzzleos/uefi-dev** | <https://github.com/puzzleos/uefi-dev> | QEMU/OVMF/swtpm UEFI Secure Boot dev environment | Reference — closest single-config analog. Single-persona dev scaffold, not a matrix. |
| **tompreston/qemu-ovmf-swtpm** | <https://github.com/tompreston/qemu-ovmf-swtpm> | Setup scripts for local QEMU+OVMF+swtpm testing | Reference starter. |
| **Noodles' Emptiness blog** | <https://www.earth.li/~noodles/blog/2024/07/qemu-uefi-testing.html> | Practitioner walkthrough of the QEMU+OVMF+swtpm setup | Reference for setup-time gotchas. |

### Academic landscape

Thin. The nexus-agents arxiv/semanticscholar/OpenAlex discovery on UEFI-emulation-testing topics returned almost nothing directly relevant — published work is dominated by **fuzzing and vulnerability discovery** (chipsec-style), not **functional conformance matrices**.

Indirectly relevant:

- Confidential-compute attestation papers (AMD SEV-SNP, Intel TDX — USENIX Security 2021–2024) inform the design of the attestation-roundtrip scenario. They establish what a correct measurement chain looks like.
- Survey papers from Eclypsium / MITRE on firmware-trust measurement give context for WHY this test surface matters.

Published work directly on persona-matrix emulation testing: **not found**. aegis-hwsim is greenfield in the academic literature.

### Commercial tools (proprietary, no direct overlap signal)

- Phoenix, Insyde, AMI — firmware-vendor test suites. Not public. Used internally; not relevant to open-source tooling.
- LaunchKey, Elemental OS, etc. — enterprise-device-management adjacent, not firmware emulation.

## Differentiation summary

aegis-hwsim's unique contributions:

1. **Persona-driven matrix keyed on real shipping hardware** — openQA matrixes on distro × version; fwupd matrixes on capsule × firmware. We matrix on **laptop-vendor × firmware-version × SB-state × TPM × lockdown**. Nothing else does this.
2. **aegis-boot-specific assertions** — signed-chain USB rescue flow, MOK enrollment recipe accuracy, kexec signature verification, attestation manifest roundtrip. These are not a subset of any existing tool's scope.
3. **Honest Linux-visible-surface scope** — we don't attempt UEFI UI automation (LAVA tried and documented that it's unworkable). Scoping down on purpose where LAVA scoped up.

## References (2026-04-18 web survey)

- QEMU + OVMF + swtpm tutorial — <https://www.earth.li/~noodles/blog/2024/07/qemu-uefi-testing.html>
- fwupd QEMU/SB integration — <https://github.com/fwupd/fwupd>
- LAVA UEFI integration docs — <https://docs.lavasoftware.org/lava/integrate-uefi.html>
- LAVA bootloader/firmware docs — <https://docs.lavasoftware.org/lava/bootloaders.html>
- Ubuntu SB testing wiki — <https://wiki.ubuntu.com/UEFI/SecureBoot/Testing>
- Ubuntu shim update test plan — <https://wiki.ubuntu.com/UEFI/SecureBoot/ShimUpdateProcess/TestPlan>
- NSA Guidance for Managing UEFI Secure Boot (Dec 2025) — <https://media.defense.gov/2025/Dec/11/2003841096/-1/-1/0/CSI_UEFI_SECURE_BOOT.PDF>
- Debian SecureBoot/VirtualMachine wiki — <https://wiki.debian.org/SecureBoot/VirtualMachine>
