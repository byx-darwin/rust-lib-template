# {{ project-name }}

[![CI](https://github.com/byx-darwin/rust-lib-template/actions/workflows/build.yml/badge.svg)](https://github.com/byx-darwin/rust-lib-template/actions/workflows/build.yml)

A Rust CLI tool built from the [rust-lib-template](https://github.com/byx-darwin/rust-lib-template).

## Quickstart

```bash
# Install
cargo install --path apps/cli

# Run
{{ project-name }} --help
{{ project-name }} run

# Generate shell completions
{{ project-name }} completions bash
```

## Development

```bash
# Install dev tools
make install-tools

# Build and test
make build
make test
make lint

# Run locally
make run
```

## Configuration

{{ project-name }} reads config from:

1. Built-in defaults
2. `$XDG_CONFIG_HOME/{{ project-name }}/config.toml`
3. Environment variables prefixed with `{{ project-name | upper_case }}_`
4. CLI flags (highest priority)

See [docs/config.md](docs/config.md) for details.

## License

MIT — see [LICENSE.md](LICENSE.md).
