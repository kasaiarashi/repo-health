use async_trait::async_trait;
use crate::github::RepoData;
use crate::Result;
use super::{Analyzer, AnalysisResult, Finding};

pub struct CiCdAnalyzer;

impl CiCdAnalyzer {
    fn detect_ci_configs(&self, tree: &[crate::github::TreeEntry]) -> Vec<String> {
        let mut configs = Vec::new();

        for entry in tree {
            if entry.entry_type != "blob" {
                continue;
            }

            if entry.path.starts_with(".github/workflows/") && entry.path.ends_with(".yml") {
                configs.push(format!("GitHub Actions: {}", entry.path));
            } else if entry.path == ".circleci/config.yml" {
                configs.push("CircleCI".to_string());
            } else if entry.path == ".travis.yml" {
                configs.push("Travis CI".to_string());
            } else if entry.path == "Jenkinsfile" {
                configs.push("Jenkins".to_string());
            } else if entry.path == ".gitlab-ci.yml" {
                configs.push("GitLab CI".to_string());
            } else if entry.path == "azure-pipelines.yml" {
                configs.push("Azure Pipelines".to_string());
            }
        }

        configs
    }
}

#[async_trait]
impl Analyzer for CiCdAnalyzer {
    fn name(&self) -> &str {
        "CI/CD"
    }

    fn weight(&self) -> f64 {
        0.20
    }

    async fn analyze(&self, repo_data: &RepoData) -> Result<AnalysisResult> {
        let mut score = 0.0;
        let mut findings = Vec::new();

        let ci_configs = self.detect_ci_configs(&repo_data.tree);

        if ci_configs.is_empty() {
            findings.push(Finding::missing("No CI/CD configuration detected"));
        } else {
            // GitHub Actions workflow
            let has_github_actions = ci_configs.iter().any(|c| c.contains("GitHub Actions"));
            if has_github_actions {
                score += 50.0;
                findings.push(Finding::positive("GitHub Actions configured"));

                // Count workflows
                let workflow_count = ci_configs.iter().filter(|c| c.contains("GitHub Actions")).count();
                if workflow_count > 1 {
                    score += 15.0;
                    findings.push(Finding::positive(format!("Multiple workflows configured ({})", workflow_count)));
                }
            }

            // Other CI systems
            let other_ci = ci_configs.iter().any(|c| !c.contains("GitHub Actions"));
            if other_ci && !has_github_actions {
                score += 40.0;
                for config in &ci_configs {
                    findings.push(Finding::positive(format!("CI configured: {}", config)));
                }
            } else if other_ci {
                for config in ci_configs.iter().filter(|c| !c.contains("GitHub Actions")) {
                    findings.push(Finding::positive(format!("Additional CI: {}", config)));
                }
            }

            // Note: We can't easily check last run or success rate without more API calls
            // For MVP, we'll give points just for having CI configured
            if !ci_configs.is_empty() {
                score += 20.0;
                findings.push(Finding::positive("CI/CD pipeline established"));
            }
        }

        let details = if ci_configs.is_empty() {
            "No CI/CD pipelines detected".to_string()
        } else {
            format!("Found {} CI/CD configuration(s)", ci_configs.len())
        };

        Ok(AnalysisResult {
            score,
            details,
            findings,
        })
    }
}
