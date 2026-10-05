# `bake-markdown`

Markdown normalization for Bake projects. It joins soft source line breaks in Markdown files while preserving explicit hard breaks, code, and block boundaries.

## Motivation

Hard-wrapped paragraphs are difficult to edit consistently. This crate uses the `socketry-markdown` syntax tree and serializer so normalization follows Markdown structure instead of changing lines with regular expressions.

## Usage

Add the crate to the project's private `bake/` package and regenerate its task links:

```sh
cargo bake --regenerate
cargo add --manifest-path bake/Cargo.toml bake-markdown
cargo bake --regenerate
```

Normalize one or more files relative to the project root:

```sh
cargo bake markdown:normalize readme.md context/guide.md
```

Rust callers can use `bake_markdown::normalize_document` directly.

## Releasing

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`, or `bump --version X.Y.Z`), then run `cargo bake cargo:release` and open a pull request. After review and merge, GitHub Actions publishes the release when the configured `crates-io` environment approves it. Follow the shared [Releasing skill](https://github.com/socketry/socketry-project-rust/blob/main/context/releasing.md) for the standard process.

## Releases

<!-- bake-readme:releases:start -->

See [releases.md](releases.md) for the full release history.

### v0.3.1

- Resolve development task dependencies to the current checkout.

- Adopt `socketry-project` 0.3.7 for shared project tasks and Markdown normalization.

- Require the aggregate test and coverage result for pull request merges.

- Avoid duplicate ordinary test runs in pull request publishing checks.

### v0.3.0

- Normalize unordered Markdown lists with hyphen markers.

### v0.2.0

- Accept Markdown paths as positional arguments to `markdown:normalize`.

<!-- bake-readme:releases:end -->

## See Also

[`socketry-markdown`](https://github.com/socketry/socketry-markdown-rust) provides the parser and serializer used by this crate.

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/socketry/bake-markdown-rust).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Read `.agents/context/index.md` to find relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. The installer preserves repository-owned `agents.md`; it does not create or regenerate that file.
