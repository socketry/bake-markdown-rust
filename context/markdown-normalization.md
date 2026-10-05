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

The task accepts one or more repeatable `--path` arguments. Paths are resolved
relative to the Bake project root:

```sh
cargo bake markdown:normalize --path readme.md --path context/guide.md
```

It writes a file only when the serialized content differs from its source.
