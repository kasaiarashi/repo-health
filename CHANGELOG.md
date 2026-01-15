# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-15

### Added
- Initial release of repo-health CLI tool
- Five analysis categories:
  - Documentation analyzer (20% weight)
  - Tests analyzer (25% weight)
  - CI/CD analyzer (20% weight)
  - Dependencies analyzer (20% weight)
  - Bus Factor analyzer (15% weight)
- Markdown report generation with shields.io badges
- Colored terminal output
- Support for both owner/repo and full GitHub URL formats
- Optional GitHub authentication (works on public repos without token)
- CLI options: --token, --output, --quiet
- Windows and Linux binary releases
- GitHub Actions automated release workflow

### Features
- Works on public repositories without authentication (60 req/hour)
- Optional token support for higher rate limits (5000 req/hour)
- Generates REPO_HEALTH.md report with detailed findings
- Weighted scoring system with letter grades (A+ to D)
- Parallel data fetching for better performance
- Robust error handling and rate limit management

[0.1.0]: https://github.com/kasaiarashi/repo-health/releases/tag/v0.1.0
