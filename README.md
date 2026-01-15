# repo-health

A CLI tool to analyze GitHub repository health and generate detailed reports.

## Features

Analyzes repositories across five key dimensions:

- **Documentation** (20%): README quality, docs folder, LICENSE, CONTRIBUTING
- **Tests** (25%): Test files, test directories, CI test integration
- **CI/CD** (20%): GitHub Actions, CircleCI, Travis, Jenkins configurations
- **Dependencies** (20%): Dependency management, maintenance status
- **Bus Factor** (15%): Contributor distribution and project sustainability

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/repo-health`.

## Usage

Set your GitHub token as an environment variable:

```bash
export GITHUB_TOKEN=ghp_your_token_here
```

Analyze a repository:

```bash
# Using owner/repo format
repo-health rust-lang/rust

# Using full GitHub URL
repo-health https://github.com/rust-lang/rust

# Specify custom output file
repo-health rust-lang/rust --output custom-report.md

# Quiet mode (no stdout, only file)
repo-health rust-lang/rust --quiet
```

## Output

The tool generates:
1. A markdown report file (default: `REPO_HEALTH.md`)
2. Terminal output with colored scores (unless --quiet is specified)
3. A shields.io badge URL for the repository

### Example Report

```markdown
# Repository Health Report

**Repository**: rust-lang/rust
**Generated**: 2026-01-15 12:00:00 UTC
**Overall Score**: 88.5/100 (A Good)

![Repo Health Badge](https://img.shields.io/badge/repo--health-good-green?style=flat-square&logo=github)

## Scores by Category
| Category | Score | Grade | Weight | Details |
|----------|-------|-------|--------|---------|
| Documentation | 90/100 | A+ | 20% | Found 5 documentation elements |
| Tests | 95/100 | A+ | 25% | Excellent test coverage |
...
```

## Requirements

- Rust 1.70 or later
- GitHub Personal Access Token with `repo` scope
- Internet connection for GitHub API access

## Authentication

Create a GitHub Personal Access Token:
1. Go to https://github.com/settings/tokens
2. Click "Generate new token (classic)"
3. Select scopes: `repo` (for private repos) or `public_repo` (for public only)
4. Copy the token
5. Set as environment variable: `export GITHUB_TOKEN=your_token`

## License

MIT
