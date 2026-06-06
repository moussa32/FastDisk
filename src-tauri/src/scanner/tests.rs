use std::fs;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use tempfile::tempdir;

use super::scan_path;

#[test]
fn scanner_handles_normal_and_empty_folders() {
    let temp = tempdir().unwrap();
    fs::create_dir(temp.path().join("empty")).unwrap();
    fs::write(temp.path().join("hello.txt"), b"hello").unwrap();

    let output = scan_path(temp.path(), Arc::new(AtomicBool::new(false))).unwrap();

    assert!(output.entries.iter().any(|entry| entry.name == "empty" && entry.is_directory));
    assert!(output.entries.iter().any(|entry| entry.name == "hello.txt" && !entry.is_directory));
    assert_eq!(output.total_files, 1);
}

#[test]
fn scanner_handles_zero_byte_and_unicode_files() {
    let temp = tempdir().unwrap();
    fs::write(temp.path().join("مرحبا.txt"), []).unwrap();

    let output = scan_path(temp.path(), Arc::new(AtomicBool::new(false))).unwrap();

    let file = output.entries.iter().find(|entry| entry.name == "مرحبا.txt").unwrap();
    assert_eq!(file.size, 0);
    assert_eq!(file.extension.as_deref(), Some("txt"));
}

#[test]
fn scanner_rejects_missing_root() {
    let temp = tempdir().unwrap();
    let missing = temp.path().join("missing");

    assert!(scan_path(missing, Arc::new(AtomicBool::new(false))).is_err());
}

#[cfg(unix)]
#[test]
fn scanner_records_symlinks_without_following_them() {
    use std::os::unix::fs::symlink;

    let temp = tempdir().unwrap();
    fs::create_dir(temp.path().join("target")).unwrap();
    symlink(temp.path().join("target"), temp.path().join("link")).unwrap();

    let output = scan_path(temp.path(), Arc::new(AtomicBool::new(false))).unwrap();

    assert!(output.entries.iter().any(|entry| entry.name == "link" && entry.is_symlink));
}
