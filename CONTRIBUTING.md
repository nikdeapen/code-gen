# Contributing

This guide outlines how contributions should be made to the codebase.

## Pull Requests

The `master` branch is protected, so every change is made on a branch & merged with a pull request.

- Each pull request is one small, focused change.
- Pull requests are squash merged, so the title becomes the commit message.
- The build workflow must pass. It runs these checks without features & again with the `rust`
  feature:

```sh
cargo fmt --check
cargo build --features rust
cargo test --features rust
cargo clippy --all-targets --features rust -- -D warnings
```

## Commits

Each commit is one small, focused change. Its message is a tag & one sentence:

```text
[rust] Added support for trait declarations.
```

The description should be a bulleted list of changes. The list of changes per PR should be
cohesive. If there are many unrelated changes they should be in separate PRs.

### Tags

- `[cargo]`: the project, typically anything focused outside of `src/`, like the manifest, the
  readme, the build workflow & version updates.
- `[core]`: the language-neutral code in `src/`, outside of `src/rust/`.
- `[rust]`: the Rust code generation in `src/rust/`.
- `[*]`: a change that spans the core & the Rust code.

## Versions

- Breaking changes bump the minor version while the crate is below `1.0`.
