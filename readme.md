# `bake-markdown`

Markdown normalization for Bake projects. It joins soft source line breaks in
Markdown files while preserving explicit hard breaks, code, and block
boundaries.

## Motivation

Hard-wrapped paragraphs are difficult to edit consistently. This crate uses
the `socketry-markdown` syntax tree and serializer so normalization follows
Markdown structure instead of changing lines with regular expressions.

## Usage

Add the crate to the project's private `bake/` package and link its tasks:

```toml
[dependencies]
bake-markdown = "0.1"
```

```rust,ignore
use bake_markdown as _;
```

Normalize one or more files relative to the project root:

```sh
cargo bake markdown:normalize readme.md context/guide.md
```

Rust callers can use `bake_markdown::normalize_document` directly.

## Releases

<!-- bake-readme:releases:start -->
See [releases.md](releases.md) for the full release history.
<!-- bake-readme:releases:end -->

## See Also

[`socketry-markdown`](https://github.com/socketry/socketry-markdown-rust) provides the parser and serializer used by this crate.

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/socketry/bake-markdown-rust).

### Agent Context

Before contributing, read `.agents/context/index.md` and the relevant context files it links. If the index is missing or out of date, run `cargo bake agent:context:install` to install context from dependencies and refresh the index.
