// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use bake_markdown as _;
use socketry_project as _;

#[cfg(not(test))]
fn main() -> bake::Result<()> {
    bake::Registry::discover()?.run()
}

#[cfg(test)]
mod tests {
    use bake::Registry;
    use std::collections::HashSet;

    #[test]
    fn registers_markdown_and_shared_project_tasks_without_duplicates() {
        let registry = Registry::discover().unwrap();
        let task_names: Vec<_> = registry.tasks().map(|task| task.name()).collect();
        let unique_task_names: HashSet<_> = task_names.iter().copied().collect();

        assert_eq!(task_names.len(), unique_task_names.len());
        assert!(unique_task_names.contains("markdown:normalize"));
        assert!(unique_task_names.contains("cargo:release"));
        assert!(unique_task_names.contains("test:coverage"));
    }
}

#[path = "bake_generated_tasks/mod.rs"]
mod bake_generated_tasks;
