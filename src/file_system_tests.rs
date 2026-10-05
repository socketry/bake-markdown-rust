// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use std::{
    collections::BTreeMap,
    io,
    path::{Path, PathBuf},
};

use super::{FileSystem, normalize_files_with};
use crate::normalize_document;

#[derive(Default)]
struct MemoryFileSystem {
    files: BTreeMap<PathBuf, String>,
    writes: Vec<PathBuf>,
    read_error: Option<io::ErrorKind>,
    write_error: Option<io::ErrorKind>,
}

impl FileSystem for MemoryFileSystem {
    fn read_to_string(&mut self, path: &Path) -> io::Result<String> {
        if let Some(error) = self.read_error {
            return Err(error.into());
        }

        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
    }

    fn write(&mut self, path: &Path, contents: &str) -> io::Result<()> {
        if let Some(error) = self.write_error {
            return Err(error.into());
        }

        self.writes.push(path.to_owned());
        self.files.insert(path.to_owned(), contents.to_owned());
        Ok(())
    }
}

#[test]
fn rejects_an_empty_path_list() {
    let mut file_system = MemoryFileSystem::default();

    let error = normalize_files_with(
        &mut file_system,
        Path::new("project"),
        &[],
        normalize_document,
    )
    .unwrap_err();

    assert_eq!(error.to_string(), "provide one or more --path arguments");
}

#[test]
fn returns_a_contextual_error_when_a_file_cannot_be_read() {
    let mut file_system = MemoryFileSystem::default();

    let error = normalize_files_with(
        &mut file_system,
        Path::new("project"),
        &[PathBuf::from("missing.md")],
        normalize_document,
    )
    .unwrap_err();

    assert!(error.to_string().contains("project/missing.md"));
    assert!(
        error
            .to_string()
            .starts_with("failed to read project/missing.md:")
    );
}

#[test]
fn returns_a_contextual_error_for_a_filesystem_read_failure() {
    let mut file_system = MemoryFileSystem {
        read_error: Some(io::ErrorKind::PermissionDenied),
        ..MemoryFileSystem::default()
    };
    file_system.files.insert(
        PathBuf::from("project/document.md"),
        "A paragraph.\n".to_owned(),
    );

    let error = normalize_files_with(
        &mut file_system,
        Path::new("project"),
        &[PathBuf::from("document.md")],
        normalize_document,
    )
    .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("failed to read project/document.md:")
    );
}

#[test]
fn writes_only_documents_that_change() {
    let mut file_system = MemoryFileSystem::default();
    file_system.files.insert(
        PathBuf::from("project/wrapped.md"),
        "A paragraph\nwith a soft break.\n".to_owned(),
    );
    file_system.files.insert(
        PathBuf::from("project/normalized.md"),
        "Already one line.\n".to_owned(),
    );

    let changed = normalize_files_with(
        &mut file_system,
        Path::new("project"),
        &[PathBuf::from("wrapped.md"), PathBuf::from("normalized.md")],
        normalize_document,
    )
    .unwrap();

    assert_eq!(changed, 1);
    assert_eq!(
        file_system.writes,
        vec![PathBuf::from("project/wrapped.md")]
    );
}

#[test]
fn returns_a_contextual_error_when_a_file_cannot_be_written() {
    let mut file_system = MemoryFileSystem {
        write_error: Some(io::ErrorKind::PermissionDenied),
        ..MemoryFileSystem::default()
    };
    file_system.files.insert(
        PathBuf::from("project/document.md"),
        "A paragraph\nwith a soft break.\n".to_owned(),
    );

    let error = normalize_files_with(
        &mut file_system,
        Path::new("project"),
        &[PathBuf::from("document.md")],
        normalize_document,
    )
    .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("failed to write project/document.md")
    );
}

#[test]
fn returns_a_contextual_error_when_markdown_cannot_be_parsed() {
    let mut file_system = MemoryFileSystem::default();
    file_system.files.insert(
        PathBuf::from("project/document.mdx"),
        "<Open></Different>".to_owned(),
    );
    let options = socketry_markdown::ParseOptions::mdx();

    let error = normalize_files_with(
        &mut file_system,
        Path::new("project"),
        &[PathBuf::from("document.mdx")],
        |source| crate::normalize_document_with_options(source, &options),
    )
    .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("failed to normalize project/document.mdx:")
    );
}
