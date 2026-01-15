use async_trait::async_trait;
use crate::github::RepoData;
use crate::Result;
use super::{Analyzer, AnalysisResult, Finding};

pub struct DependenciesAnalyzer;

impl DependenciesAnalyzer {
    fn find_dependency_files(&self, tree: &[crate::github::TreeEntry]) -> Vec<String> {
        let mut files = Vec::new();

        for entry in tree {
            if entry.entry_type != "blob" {
                continue;
            }

            match entry.path.as_str() {
                "Cargo.toml" => files.push("Cargo.toml (Rust)".to_string()),
                "package.json" => files.push("package.json (Node.js)".to_string()),
                "requirements.txt" => files.push("requirements.txt (Python)".to_string()),
                "Pipfile" => files.push("Pipfile (Python)".to_string()),
                "pyproject.toml" => files.push("pyproject.toml (Python)".to_string()),
                "go.mod" => files.push("go.mod (Go)".to_string()),
                "pom.xml" => files.push("pom.xml (Java/Maven)".to_string()),
                "build.gradle" | "build.gradle.kts" => files.push("Gradle (Java)".to_string()),
                "Gemfile" => files.push("Gemfile (Ruby)".to_string()),
                _ => {}
            }
        }

        files
    }

    fn estimate_dependency_count(&self, tree: &[crate::github::TreeEntry]) -> usize {
        // Simple heuristic: assume projects with dep files have dependencies
        let dep_files = self.find_dependency_files(tree);

        if dep_files.is_empty() {
            return 0;
        }

        // For now, we'll use a rough estimate based on project size
        // In a full implementation, we'd fetch and parse these files
        let code_files: usize = tree.iter()
            .filter(|e| {
                e.entry_type == "blob" && (
                    e.path.ends_with(".rs") || e.path.ends_with(".js") ||
                    e.path.ends_with(".ts") || e.path.ends_with(".py") ||
                    e.path.ends_with(".go") || e.path.ends_with(".java")
                )
            })
            .count();

        // Rough estimate: small project < 50 files, medium < 200, large > 200
        if code_files < 50 {
            5
        } else if code_files < 200 {
            15
        } else {
            30
        }
    }
}

#[async_trait]
impl Analyzer for DependenciesAnalyzer {
    fn name(&self) -> &str {
        "Dependencies"
    }

    fn weight(&self) -> f64 {
        0.20
    }

    async fn analyze(&self, repo_data: &RepoData) -> Result<AnalysisResult> {
        let mut score = 0.0;
        let mut findings = Vec::new();

        let dep_files = self.find_dependency_files(&repo_data.tree);

        if dep_files.is_empty() {
            findings.push(Finding::missing("No dependency files detected"));
        } else {
            score += 20.0;
            findings.push(Finding::positive(format!(
                "Dependency management: {}",
                dep_files.join(", ")
            )));

            let dep_count = self.estimate_dependency_count(&repo_data.tree);
            if dep_count > 0 {
                score += 20.0;
                findings.push(Finding::positive(format!(
                    "Estimated ~{} dependencies",
                    dep_count
                )));
            }

            // For MVP, we'll give a moderate score for having dependency management
            // In a full implementation, we would:
            // 1. Fetch the dependency file contents
            // 2. Parse them to get actual dependencies
            // 3. Query package registries for latest versions
            // 4. Compare current vs latest versions

            // Assume reasonable maintenance if the repo is active
            let is_archived = repo_data.repository.archived.unwrap_or(false);
            if !is_archived {
                score += 40.0;
                findings.push(Finding::positive("Repository is actively maintained"));
            } else {
                findings.push(Finding::warning("Repository is archived - dependencies may be outdated"));
            }
        }

        let details = if dep_files.is_empty() {
            "No dependency management detected".to_string()
        } else {
            format!("Found {} dependency file(s)", dep_files.len())
        };

        Ok(AnalysisResult {
            score,
            details,
            findings,
        })
    }
}
