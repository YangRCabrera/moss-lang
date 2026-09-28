# Moss

Moss is a statically typed, expression-oriented functional language with explicit
mutable references, algebraic data types, pattern matching, and no null value.

> Ordinary values are immutable. Recoverable uncertainty is represented in types.
> Mutation is explicit. Programmer assertions may fail catastrophically.

This project is in its very early stages. There is no working compiler yet — the
current focus is nailing down the language's semantics before building the
implementation.

## Status

🌱 Pre-alpha. Expect nothing to work.

## Project Layout

Still a skeleton at this stage. Entries marked *(planned)* don't exist yet —
they're staked out for the pieces of a compiler pipeline.

```
moss-lang/
├── src/
│   ├── main.rs
│   ├── lexer/      (planned)
│   ├── parser/     (planned)
│   ├── ast/        (planned)
│   └── runtime/    (planned)
├── tests/          (planned)
├── docs/
├── build.rs        (planned)
├── Cargo.toml
├── cog.toml
├── .githooks/
└── .github/
```

## Documentation

- [Semantic Constitution](docs/Semantic%20Constitution.md) — the core semantic
  rules of the language, kept stable even as surface syntax changes.

## Getting Started

<!-- TODO: build/run instructions once there's something to build or run -->

```sh
cargo build
```

## Contributing

<!-- TODO: contribution guidelines -->

Commits follow [Conventional Commits](https://www.conventionalcommits.org/) and
are checked via git hooks (see [.githooks/](.githooks/)). Versioning is managed
with [cocogitto](https://docs.cocogitto.io/) (see [cog.toml](cog.toml)).

After cloning, install cocogitto and enable the hooks:

```sh
cargo install --locked cocogitto
git config core.hooksPath .githooks
```

## License

[MIT](LICENSE)
