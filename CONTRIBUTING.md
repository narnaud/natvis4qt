# Contributing to **Natvis4Qt**

Thank you for your interest in contributing to **natvis4qt**. Your contributions are very welcome in the form of bug reports, feature requests, documentation improvements, and code changes.

This guide will help you understand how to contribute to the project.

## Pull request process

1. Ensure to install [`pre-commit`](https://pre-commit.com/) before doing any pull requests

```
pip install pre-commit
pre-commit install --hook-type commit-msg
```

2. Use [Conventional Commits](https://www.conventionalcommits.org/) for your commit messages

```
<type>[optional scope]: <description>

[optional body]
```

For example:

```
feat(clink): Add completion for environment names
```

3. Keep a linear git history, remove any merge commits but prefer rebasing on top of `main`

## Code contributions

### Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust installed on your machine. You can download it from rustup.rs.
- Familiarity with Git and GitHub.

### Code contribution

Before any rust code contributions:

- Run `cargo fmt`
- Run `cargo clippy`
