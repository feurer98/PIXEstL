//! CLI integration tests using assert_cmd
//!
//! These tests exercise the `pixestl` binary end-to-end, verifying argument
//! parsing, error messages, and successful generation workflows.

use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write as _;

/// Creates a minimal palette JSON temp file (Red + White).
fn test_palette_file() -> tempfile::NamedTempFile {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    let json = r##"{
  "#FF0000": {
    "name": "Red",
    "active": true,
    "layers": {
      "5": { "H": 0, "S": 100, "L": 50 },
      "3": { "H": 0, "S": 100, "L": 65 }
    }
  },
  "#FFFFFF": {
    "name": "White",
    "active": true,
    "layers": {
      "5": { "H": 0, "S": 0, "L": 100 }
    }
  }
}"##;
    write!(f, "{json}").unwrap();
    f
}

/// Writes a tiny 2x2 red PNG to a temp file.
fn test_image_file() -> tempfile::NamedTempFile {
    use image::{ImageBuffer, Rgba};
    let buf = ImageBuffer::from_fn(4, 4, |_, _| Rgba([255u8, 0, 0, 255]));
    let f = tempfile::NamedTempFile::with_suffix(".png").unwrap();
    buf.save(f.path()).unwrap();
    f
}

// ── Help and version ────────────────────────────────────────────────────────

#[test]
fn test_help_exits_successfully() {
    Command::cargo_bin("pixestl")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate color lithophanes"));
}

#[test]
fn test_version_exits_successfully() {
    Command::cargo_bin("pixestl")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("pixestl"));
}

// ── Missing required arguments ──────────────────────────────────────────────

#[test]
fn test_missing_palette_exits_with_error() {
    Command::cargo_bin("pixestl")
        .unwrap()
        .args(["--input", "dummy.png", "--output", "out.zip"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--palette"));
}

#[test]
fn test_missing_input_and_output_exits_with_error() {
    let palette = test_palette_file();
    Command::cargo_bin("pixestl")
        .unwrap()
        .args(["--palette", palette.path().to_str().unwrap()])
        .assert()
        .failure();
}

// ── Nonexistent input file ──────────────────────────────────────────────────

#[test]
fn test_nonexistent_image_reports_error() {
    let palette = test_palette_file();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("out.zip");

    Command::cargo_bin("pixestl")
        .unwrap()
        .args([
            "--input",
            "/tmp/does_not_exist_pixestl_test.png",
            "--palette",
            palette.path().to_str().unwrap(),
            "--output",
            out.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

// ── Nonexistent palette file ────────────────────────────────────────────────

#[test]
fn test_nonexistent_palette_reports_error() {
    let img = test_image_file();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("out.zip");

    Command::cargo_bin("pixestl")
        .unwrap()
        .args([
            "--input",
            img.path().to_str().unwrap(),
            "--palette",
            "/tmp/no_such_palette_pixestl.json",
            "--output",
            out.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

// ── Successful generation to ZIP ────────────────────────────────────────────

#[test]
fn test_successful_zip_generation() {
    let img = test_image_file();
    let palette = test_palette_file();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("result.zip");

    Command::cargo_bin("pixestl")
        .unwrap()
        .args([
            "--input",
            img.path().to_str().unwrap(),
            "--palette",
            palette.path().to_str().unwrap(),
            "--output",
            out.to_str().unwrap(),
            "--no-texture",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Fertig!"));

    // Verify the ZIP was actually created and is valid
    let zip_bytes = std::fs::read(&out).unwrap();
    let archive = zip::ZipArchive::new(std::io::Cursor::new(zip_bytes)).unwrap();
    assert!(!archive.is_empty(), "ZIP should contain at least one layer");
}

// ── Successful generation to directory ──────────────────────────────────────

#[test]
fn test_successful_dir_generation() {
    let img = test_image_file();
    let palette = test_palette_file();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("stl_output");

    Command::cargo_bin("pixestl")
        .unwrap()
        .args([
            "--input",
            img.path().to_str().unwrap(),
            "--palette",
            palette.path().to_str().unwrap(),
            "--output",
            out.to_str().unwrap(),
            "--no-texture",
        ])
        .assert()
        .success();

    // Verify STL files were written to the directory
    assert!(out.exists(), "output directory should exist");
    let stl_files: Vec<_> = std::fs::read_dir(&out)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "stl"))
        .collect();
    assert!(!stl_files.is_empty(), "directory should contain STL files");
}

// ── Successful generation to 3MF ────────────────────────────────────────────

#[test]
fn test_successful_3mf_generation() {
    let img = test_image_file();
    let palette = test_palette_file();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("result.3mf");

    Command::cargo_bin("pixestl")
        .unwrap()
        .args([
            "--input",
            img.path().to_str().unwrap(),
            "--palette",
            palette.path().to_str().unwrap(),
            "--output",
            out.to_str().unwrap(),
            "--no-texture",
        ])
        .assert()
        .success();

    assert!(out.exists(), "3MF file should exist");
    assert!(
        std::fs::metadata(&out).unwrap().len() > 0,
        "3MF file should not be empty"
    );
}

// ── Palette info mode ───────────────────────────────────────────────────────

#[test]
fn test_palette_info_mode() {
    let palette = test_palette_file();

    Command::cargo_bin("pixestl")
        .unwrap()
        .args([
            "--palette",
            palette.path().to_str().unwrap(),
            "--palette-info",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Palette Information"));
}

// ── Calibration mode ────────────────────────────────────────────────────────

#[test]
fn test_calibrate_mode() {
    let palette = test_palette_file();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("calibration.zip");

    Command::cargo_bin("pixestl")
        .unwrap()
        .args([
            "--palette",
            palette.path().to_str().unwrap(),
            "--output",
            out.to_str().unwrap(),
            "--calibrate",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Kalibrierungs"));

    assert!(out.exists(), "calibration output should exist");
}
