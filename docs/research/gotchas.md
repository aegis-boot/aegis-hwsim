# Confirmed gotchas

Captured from prior-art survey on 2026-04-18. Each entry links to its source so future contributors can verify.

## 1. LAVA documented that UEFI UI automation is unworkable

> "UEFI automation proved to be unworkable in automation due to complexity of the sequences and the changes in error handling between levels of the same menus."

Source: [LAVA UEFI integration docs](https://docs.lavasoftware.org/lava/integrate-uefi.html).

**Implication for aegis-hwsim:** This validates our "Linux-visible surface only" scope. We deliberately don't try to automate the UEFI boot-menu UI (Lenovo's blue-screen MOK Manager, Dell F12, HP Fast Boot). Scope down on purpose where LAVA scoped up and documented the failure.

## 2. QEMU `-smbios` is userspace-visible only

Kernel vendor-quirk paths (`thinkpad_acpi`, `dell-laptop`, `framework_laptop`) don't trigger on DMI alone — they match on PCI IDs + ACPI SSDT tables, which QEMU's `-smbios` doesn't spoof.

Source: QEMU docs + recurring LKML threads on DMI-based quirk matching (verifiable via `git log --grep=thinkpad_acpi` in the Linux kernel tree).

**Implication for aegis-hwsim:** Our README must state that personas test what aegis-boot reads from `/sys/class/dmi/id/*`, not what the kernel vendor-hooks for laptop-specific behavior. Operators should not expect thinkpad_acpi code paths to be exercised by a `lenovo-*` persona.

## 3. swtpm 0.7–0.8 had PCR-extend race issues on fast reboots

PCR extend operations during early boot could race with swtpm socket-readiness. Fixed iteratively through swtpm 0.7 → 0.8 series.

Source: [swtpm CHANGES](https://github.com/stefanberger/swtpm/blob/master/CHANGES) (grep for PCR-related entries in 0.7 / 0.8 series).

**Implication for aegis-hwsim:** Pin swtpm ≥ 0.8.2 in dev docs + CI. Document in README requirements.

## 4. OVMF variant sprawl is a real maintenance tax

Fedora ships ~6 OVMF variants (`OVMF_CODE.fd`, `OVMF_CODE_4M.fd`, `OVMF_CODE_4M.secboot.fd`, `OVMF_VARS.fd`, `OVMF_VARS_4M.fd`, `OVMF_VARS_4M.ms.fd`, ...). Debian has a different split. Keeping fixtures matched to distro OVMF packaging is a recurring issue.

Source: observable via `apt-cache show ovmf` on Debian/Ubuntu; `dnf repoquery -l edk2-ovmf` on Fedora.

**Implication for aegis-hwsim:** Snap to Debian packaging (CI runs on Ubuntu). Document the variant-path resolution in a single place (`src/ovmf.rs` once implemented) so future distro support is one function per distro.

## 5. Emulation-vs-hardware delta ≈ 30–40% residual bugs only found on metal

fwupd/LVFS empirical data puts QEMU's coverage of capsule-flow bugs at ~60–70%. The rest are EC / firmware-vendor-specific and only reproduce on physical hardware.

Source: [Richard Hughes](https://github.com/hughsie) / [Mario Limonciello](https://github.com/superm1) conference talks (LVFS + fwupd maintainers).

**Implication for aegis-hwsim:** Our scope is narrower (USB rescue-stick signed-chain flow, not capsule updates), so we estimate ~80% coverage of aegis-boot's testable failure modes. But **real-hardware shakedown still required** for the remaining 20%. This is explicitly noted in [aegis-boot#226](https://github.com/williamzujkowski/aegis-boot/issues/226) and in our README.

## 6. OVMF "setup mode" VARs requires the right template

Easy mistake: using the MS-enrolled VARs template while testing key-enrollment flows. Looks like the flow works, but you're just observing a no-op because keys were pre-enrolled.

Source: [Debian SecureBoot/VirtualMachine wiki](https://wiki.debian.org/SecureBoot/VirtualMachine), [Ubuntu UEFI/OVMF wiki](https://wiki.ubuntu.com/UEFI/OVMF).

**Implication for aegis-hwsim:** `ovmf_variant: setup_mode` in the persona YAML must pick a cleanly empty-state VARs file. Add a validation check that refuses to load `setup_mode` personas without a matching setup-mode VARs path set in runner config.

## 7. kexec signature verification is kernel-version-sensitive

The kexec_file_load signature-verification path interacts non-obviously with kernel lockdown mode and module-signing config. Same signed kernel can succeed under lockdown=integrity and fail under lockdown=confidentiality.

Source: `Documentation/admin-guide/LSM/Yama.rst` + `kernel/kexec_file.c` in the Linux tree; LWN articles on lockdown mode.

**Implication for aegis-hwsim:** `kernel.lockdown` in the persona YAML must be able to differentiate `integrity` from `confidentiality` tests. The scenario matrix should include both settings for signed and unsigned kernels.
