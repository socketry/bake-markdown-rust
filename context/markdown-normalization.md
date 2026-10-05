# Markdown Normalization

`bake-markdown` exposes `normalize_document` for Rust callers and the
`markdown:normalize` Bake task for project files. Normalization parses
CommonMark, GitHub Flavored Markdown, YAML and TOML front matter, and math,
then serializes it with soft source line breaks joined as spaces. This keeps
inline code and other parsed syntax such as tables, task list state, footnotes,
strikethrough, code blocks, and math in the syntax tree for serialization.
Rust callers can select another syntax with `normalize_document_with_options`,
such as MDX.

The serializer preserves explicit Markdown hard breaks, code contents, and
block boundaries. YAML and TOML front matter are parsed as front matter so
their delimiters and content remain intact. The serializer canonicalizes
line endings to LF and emits a trailing newline for non-empty documents. The
normalizer does not wrap paragraphs to a target line width.

The task accepts one or more positional paths, resolved relative to the Bake
project root. For example, the shell expands a glob before Bake receives the
paths:

```sh
cargo bake markdown:normalize **/*.md
```

Use `::` before another task because the variadic path list consumes all
positional values up to the end of the command:

```sh
cargo bake markdown:normalize **/*.md :: null
```

It writes a file only when the serialized content differs from its source.
