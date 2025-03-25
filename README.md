_This is a template for [cargo-generate](https://cargo-generate.github.io/cargo-generate/)._
_Use with `cargo generate prabirshrestha/salvo-template`._

# {{project-name}}

My web project.

## Building

```bash
cargo xtask build
```

You can also use `x` as a shorthand for `cargo xtask`. For example: `cargo x build`.

## Running

Make sure to install [systemfd](https://github.com/irmen/systemfd) with `cargo install systemfd` and [watchexec](https://github.com/watchexec/watchexec) with `cargo install systemfd watchexec-cli`.

```bash
cargo xtask dev
```

or

```
cargo x dev
```

## Api Docs

Navigate to [http://localhost:8000/api-doc](http://localhost:8000/api-doc) to view the interactive API documentation.
