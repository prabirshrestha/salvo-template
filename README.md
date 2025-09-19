_This is a template for [cargo-generate](https://cargo-generate.github.io/cargo-generate/)._
_Use with `cargo generate prabirshrestha/salvo-template`._

# {{project-name}}

My web project.

## Requirements

- Rust and Cargo
- [Tailwind CSS Standalone CLI](https://github.com/tailwindlabs/tailwindcss/releases/tag/v4.0.17). For more information refer to [documentation](https://tailwindcss.com/blog/standalone-cli).

## Building

```bash
cargo build
```

or

```bash
cargo xtask build
```

## Running

```bash
cargo run
```

## Running with watch mode

```bash
cargo watch -x run
```

or

```bash
cargo xtask dev
```

To override the default host and port, use the `--host` and `--port` flags:

```bash
cargo xtask dev --host 127.0.0.1 --port 8080
```

## Api Docs

Navigate to [http://localhost:8000/api-doc](http://localhost:8000/api-doc) to view the interactive API documentation.
