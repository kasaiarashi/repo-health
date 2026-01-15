use async_trait::async_trait;
use crate::github::RepoData;
use crate::Result;
use super::{Analyzer, AnalysisResult, Finding};

pub struct TestsAnalyzer;

impl TestsAnalyzer {
    fn is_test_file(&self, path: &str) -> bool {
        // Rust
        if path.starts_with("tests/") || path.contains("_test.rs") || path.contains("/test_") {
            return true;
        }

        // JavaScript/TypeScript
        if path.contains(".test.") || path.contains(".spec.") || path.contains("__tests__") {
            return true;
        }

        // Python
        if path.starts_with("test_") || path.ends_with("_test.py") || path.contains("/test/") {
            return true;
        }

        // Go
        if path.ends_with("_test.go") {
            return true;
        }

        // Java
        if path.contains("/test/") && path.ends_with(".java") {
            return true;
        }

        false
    }

    fn has_test_directory(&self, tree: &[crate::github::TreeEntry]) -> bool {
        tree.iter().any(|entry| {
            entry.path == "tests" || entry.path == "test"
            || entry.path == "__tests__" || entry.path == "spec"
        })
    }
}

#[async_trait]
impl Analyzer for TestsAnalyzer {
    fn name(&self) -> &str {
        "Tests"
    }

    fn weight(&self) -> f64 {
        0.25
    }

    async fn analyze(&self, repo_data: &RepoData) -> Result<AnalysisResult> {
        let mut score = 0.0;
        let mut findings = Vec::new();

        // Check for test directory
        if self.has_test_directory(&repo_data.tree) {
            score += 40.0;
            findings.push(Finding::positive("Test directory exists"));
        } else {
            findings.push(Finding::missing("No dedicated test directory found"));
        }

        // Count test files
        let test_file_count = repo_data.tree.iter()
            .filter(|entry| entry.entry_type == "blob" && self.is_test_file(&entry.path))
            .count();

        if test_file_count >= 5 {
            score += 20.0;
            findings.push(Finding::positive(format!("Found {} test files", test_file_count)));

            if test_file_count >= 10 {
                score += 10.0;
                findings.push(Finding::positive("Extensive test coverage (10+ test files)"));
            }
        } else if test_file_count > 0 {
            findings.push(Finding::warning(format!("Only {} test files found", test_file_count)));
        } else {
            findings.push(Finding::missing("No test files detected"));
        }

        // Check for CI test execution (simplified check)
        let has_ci = repo_data.tree.iter().any(|entry| {
            entry.path.starts_with(".github/workflows/") ||
            entry.path == ".circleci/config.yml" ||
            entry.path == ".travis.yml"
        });

        if has_ci {
            score += 20.0;
            findings.push(Finding::positive("CI configured (likely includes tests)"));
        }

        // Check for coverage badge in README
        if let Some(readme) = &repo_data.readme_content {
            if readme.contains("coverage") && (readme.contains("badge") || readme.contains("shields.io")) {
                score += 10.0;
                findings.push(Finding::positive("Coverage badge found in README"));
            }
        }

        let details = format!(
            "Detected {} test files across the repository. {}",
            test_file_count,
            if test_file_count >= 10 { "Excellent test coverage" }
            else if test_file_count >= 5 { "Good test coverage" }
            else if test_file_count > 0 { "Limited test coverage" }
            else { "No tests found" }
        );

        Ok(AnalysisResult {
            score,
            details,
            findings,
        })
    }
}
