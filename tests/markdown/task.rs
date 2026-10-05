// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use std::fs;

use bake::Registry;
use bake_markdown as _;

#[test]
fn registers_and_normalizes_multiple_files_from_the_project_root() {
    let directory = tempfile::tempdir().unwrap();
    fs::write(
        directory.path().join("first.md"),
        "First line\ncontinues.\n",
    )
    .unwrap();
    fs::write(
        directory.path().join("second.md"),
        "Second line\ncontinues.\n",
    )
    .unwrap();

    let registry = Registry::discover().unwrap();
    assert!(
        registry
            .tasks()
            .any(|task| task.name() == "markdown:normalize")
    );
    let help = registry.help(Some("markdown:normalize")).unwrap();
    assert!(help.contains("paths..."));
    assert!(help.contains("Markdown files, relative to the project root."));
    let mut context = registry.context(directory.path());
    let result = context
        .call("markdown:normalize", &["first.md", "second.md"])
        .unwrap();

    assert_eq!(result.as_str(), Some("Normalized 2 of 2 Markdown files"));
    assert_eq!(
        fs::read_to_string(directory.path().join("first.md")).unwrap(),
        "First line continues.\n"
    );
    assert_eq!(
        fs::read_to_string(directory.path().join("second.md")).unwrap(),
        "Second line continues.\n"
    );
}

#[test]
fn rejects_a_task_call_without_paths() {
    let directory = tempfile::tempdir().unwrap();
    let mut context = Registry::discover().unwrap().context(directory.path());

    let error = context.call("markdown:normalize", &[]).unwrap_err();

    assert!(error.to_string().contains("provide one or more file paths"));
}

#[test]
fn stops_collecting_paths_at_the_task_chain_separator() {
    let directory = tempfile::tempdir().unwrap();
    fs::write(
        directory.path().join("first.md"),
        "First line\ncontinues.\n",
    )
    .unwrap();
    fs::write(
        directory.path().join("second.md"),
        "Second line\ncontinues.\n",
    )
    .unwrap();

    let arguments =
        ["markdown:normalize", "first.md", "second.md", "::", "null"].map(str::to_owned);
    let output = Registry::discover()
        .unwrap()
        .run_arguments(directory.path(), &arguments)
        .unwrap();

    assert_eq!(output, "");
    assert_eq!(
        fs::read_to_string(directory.path().join("first.md")).unwrap(),
        "First line continues.\n"
    );
    assert_eq!(
        fs::read_to_string(directory.path().join("second.md")).unwrap(),
        "Second line continues.\n"
    );
}
