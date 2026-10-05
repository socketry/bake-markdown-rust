# Releases

## v0.3.1

- Resolve development task dependencies to the current checkout.

- Adopt `socketry-project` 0.3.7 for shared project tasks and Markdown normalization.

- Require the aggregate test and coverage result for pull request merges.

- Avoid duplicate ordinary test runs in pull request publishing checks.

## v0.3.0

- Normalize unordered Markdown lists with hyphen markers.

## v0.2.0

- Accept Markdown paths as positional arguments to `markdown:normalize`.

## v0.1.0

- Add the `markdown:normalize` task to join soft source line breaks in Markdown files while preserving explicit hard breaks, code, and block boundaries.
