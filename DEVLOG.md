# Devlog

Informal notes on the development of Moss, newest first.

## Project scaffold

- Set up two code example files to test and refint the syntax ([FizzBuzz](examples\FizzBuzz.moss)
  and [Fibonacci](examples\Fibonacci.moss)).
- Most notably, the examples helped establish specific `lambda` syntax
  and exposed issues with the `*` prefix as sugar for `Ref.get`/`Ref.set`.
- [Semantic Constitution](docs/Semantic%20Constitution.md) was updated to conform
  to newly established rules.

## Project scaffold

- Set up the Rust crate (edition 2024), CI (fmt, clippy, tests, commit-message
  check), and Conventional Commits tooling via cocogitto and git hooks.
- Drafted the [Semantic Constitution](docs/Semantic%20Constitution.md): the core
  semantic rules the language must uphold regardless of surface syntax.
- No compiler code yet.
