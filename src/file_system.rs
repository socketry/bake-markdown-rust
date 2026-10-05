// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use bake::{Error, Result};
use socketry_markdown::message::Message;

trait FileSystem {
    fn read_to_string(&mut self, path: &Path) -> io::Result<String>;
    fn write(&mut self, path: &Path, contents: &str) -> io::Result<()>;
}

struct LocalFileSystem;

impl FileSystem for LocalFileSystem {
    fn read_to_string(&mut self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn write(&mut self, path: &Path, contents: &str) -> io::Result<()> {
        fs::write(path, contents)
    }
}

pub(super) fn normalize_files(root: &Path, paths: &[PathBuf]) -> Result<usize> {
    normalize_files_with(&mut LocalFileSystem, root, paths, crate::normalize_document)
}

fn normalize_files_with(
    file_system: &mut impl FileSystem,
    root: &Path,
    paths: &[PathBuf],
    normalize: impl Fn(&str) -> std::result::Result<String, Message>,
) -> Result<usize> {
    if paths.is_empty() {
        return Err(Error::new("provide one or more file paths"));
    }

    let mut changed = 0;

    for path in paths {
        let full_path = root.join(path);
        let source = file_system.read_to_string(&full_path).map_err(|error| {
            Error::new(format!("failed to read {}: {error}", full_path.display()))
        })?;
        let normalized = normalize(&source).map_err(|error| {
            Error::new(format!(
                "failed to normalize {}: {error}",
                full_path.display()
            ))
        })?;

        if normalized != source {
            file_system
                .write(&full_path, &normalized)
                .map_err(|error| {
                    Error::new(format!("failed to write {}: {error}", full_path.display()))
                })?;
            changed += 1;
        }
    }

    Ok(changed)
}

#[cfg(test)]
#[path = "file_system_tests.rs"]
mod tests;
