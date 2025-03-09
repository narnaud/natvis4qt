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

## Natvis contributions

To contribute new visualiser (or improve existing one), feel free to use the small executable in the `natvis` directory:

- Open the `CMakeLists.txt` in Visual Studio
- Feel free to add a type if it's missing
- Put a breakpoint at the `return` in main
- You can edit the natvis files and see the results directly

## Code contributions

### Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust installed on your machine. You can download it from rustup.rs.
- Familiarity with Git and GitHub.

### Code contribution

Before any rust code contributions:

- Run `cargo fmt`
- Run `cargo clippy`
