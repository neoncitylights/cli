# Markdown Template

[![CI](https://img.shields.io/github/actions/workflow/status/neoncitylights/mdbook/.github/workflows/ci.yml?style=flat-square)](https://github.com/neoncitylights/mdbook/actions/workflows/ci.yml)
[![GitHub deployments](https://img.shields.io/github/deployments/neoncitylights/mdbook/github-pages?label=docs&style=flat-square)](https://neoncitylights.github.io/mdbook/)

A repository template that builds a static documentation site using Markdown and mdBook.

Make a change, commit, push, and see the updates live on your GitHub Pages site!

## Features

- [x] Linting markdown files with [`markdownlint-cli2`](https://github.com/DavidAnson/markdownlint-cli2)
- [x] Auto-generated documentation using [mdBook](https://github.com/rust-lang/mdBook) and [GitHub Pages](https://docs.github.com/en/pages)
- [x] Continuous deployment with [GitHub Actions](https://github.com/features/actions), with fast-build & deploy times (< 30 seconds)
- [x] Dependency updates with [Dependabot](https://github.com/dependabot)
- [x] Remote development with [GitHub Codespaces](https://github.com/features/codespaces)

## Configure

| Tool              | File                                                     | Documentation                                                              |
| ----------------- | -------------------------------------------------------- | -------------------------------------------------------------------------- |
| mdBook            | [`book.toml`](./book.toml)                               | [docs](https://rust-lang.github.io/mdBook/format/configuration/index.html) |
| mdBook            | [`SUMMARY.md`](./src/SUMMARY.md)                         | [docs](https://rust-lang.github.io/mdBook/format/summary.html)             |
| markdownlint-cli2 | [`.markdownlint-cli2.jsonc`](./.markdownlint-cli2.jsonc) | [docs](https://github.com/DavidAnson/markdownlint-cli2/blob/main/README.md#markdownlint-cli2jsonc), [example file](https://github.com/DavidAnson/markdownlint-cli2/blob/main/test/markdownlint-cli2-jsonc-example/.markdownlint-cli2.jsonc) |

## Build locally

```bash
git clone https://github.com/neoncitylights/mdbook.git
cd mdbook
npm install
npm run build
```

### System requirements

Be sure to also install the following tools:
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Rust package manager)
- [mdBook](https://rust-lang.github.io/mdBook/guide/installation.html) (Static site generator with Markdown)

## Lint markdown files

- To lint markdown: `npm run lint`
- To auto-fix lint errors: `npm run lint:fix`
