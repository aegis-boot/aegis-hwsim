//! Integration tests for the E1 loader guards. Each fixture under
//! `tests/fixtures/bad-*.yaml` is copied into a fresh `personas/` dir
//! (created in a temp tree) and `load_all()` must return the specific
//! `LoadError` variant that guard was designed to catch.
//!
//! Tests are one-per-guard so a regression that breaks a single guard
//! lands as a single failing test with a clear name.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use aegis_hwsim::loader::{load_all, LoadError, LoadOptions};
use std::fs;
use std::path::{Path, PathBuf};

fn fixture_path(filename: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("fixtures");
    p.push(filename);
    p
}

fn stage_fixture(fixture: &str) -> (tempfile::TempDir, LoadOptions) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let personas = tmp.path().join("personas");
    let firmware = tmp.path().join("firmware");
    fs::create_dir_all(&personas).expect("mkdir personas");
    fs::create_dir_all(&firmware).expect("mkdir firmware");
    let body = fs::read_to_string(fixture_path(fixture)).expect("read fixture");
    let dest = personas.join(fixture);
    fs::write(&dest, body).expect("write fixture copy");
    let opts = LoadOptions {
        personas_dir: personas,
        firmware_root: firmware,
    };
    (tmp, opts)
}

fn fixture_abs_path(opts: &LoadOptions, fixture: &str) -> PathBuf {
    opts.personas_dir.join(fixture)
}

fn assert_path_matches(got: &Path, opts: &LoadOptions, fixture: &str) {
    let expected = fixture_abs_path(opts, fixture);
    assert_eq!(got, expected, "error path should point at the fixture copy");
}

#[test]
fn missing_schema_version_rejected_with_parse_error() {
    let (_tmp, opts) = stage_fixture("bad-missing-schema-version.yaml");
    let err = load_all(&opts).expect_err("expected LoadError");
    match err {
        LoadError::Parse { path, .. } => {
            assert_path_matches(&path, &opts, "bad-missing-schema-version.yaml");
        }
        other => panic!("expected Parse, got {other:?}"),
    }
}

#[test]
fn placeholder_token_rejected() {
    let (_tmp, opts) = stage_fixture("bad-placeholder-token.yaml");
    let err = load_all(&opts).expect_err("expected LoadError");
    match err {
        LoadError::Placeholder { path, field } => {
            assert_path_matches(&path, &opts, "bad-placeholder-token.yaml");
            assert_eq!(field, "display_name");
        }
        other => panic!("expected Placeholder, got {other:?}"),
    }
}

#[test]
fn id_filename_drift_rejected() {
    let (_tmp, opts) = stage_fixture("bad-id-filename-drift.yaml");
    let err = load_all(&opts).expect_err("expected LoadError");
    match err {
        LoadError::IdMismatch {
            path,
            yaml_id,
            filename_stem,
        } => {
            assert_path_matches(&path, &opts, "bad-id-filename-drift.yaml");
            assert_eq!(yaml_id, "wrong-id-here");
            assert_eq!(filename_stem, "bad-id-filename-drift");
        }
        other => panic!("expected IdMismatch, got {other:?}"),
    }
}

#[test]
fn quirk_tag_with_uppercase_rejected() {
    let (_tmp, opts) = stage_fixture("bad-quirk-tag-syntax.yaml");
    let err = load_all(&opts).expect_err("expected LoadError");
    match err {
        LoadError::QuirkTag { path, tag } => {
            assert_path_matches(&path, &opts, "bad-quirk-tag-syntax.yaml");
            assert_eq!(tag, "BAD_TAG_UPPER");
        }
        other => panic!("expected QuirkTag, got {other:?}"),
    }
}

#[test]
fn custom_keyring_outside_root_rejected() {
    let (_tmp, opts) = stage_fixture("bad-custom-keyring-traversal.yaml");
    let err = load_all(&opts).expect_err("expected LoadError");
    assert!(
        matches!(err, LoadError::CustomKeyringOutsideRoot { .. }),
        "expected CustomKeyringOutsideRoot, got {err:?}"
    );
}
