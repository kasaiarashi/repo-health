use async_trait::async_trait;
use crate::github::RepoData;
use crate::Result;
use super::{Analyzer, AnalysisResult, Finding};

pub struct DocumentationAnalyzer;

#[async_trait]
impl Analyzer for DocumentationAnalyzer {
    fn name(&self) -> &str {
        "Documentation"
    }

    fn weight(&self) -> f64 {
        0.20
    }

    async fn analyze(&self, repo_data: &RepoData) -> Result<AnalysisResult> {
        let mut score = 0.0;
        let mut findings = Vec::new();

        // Check README exists and quality
        if let Some(readme) = &repo_data.readme_content {
            score += 40.0;
            findings.push(Finding::positive("README.md exists"));

            // Check README length
            if readme.len() > 500 {
                score += 10.0;
                findings.push(Finding::positive("README has substantial content (>500 chars)"));
            } else {
                findings.push(Finding::warning("README is quite short (<500 chars)"));
            }

            // Check for sections (markdown headers)
            if readme.contains("##") {
                score += 10.0;
                findings.push(Finding::positive("README has sections"));
            } else {
                findings.push(Finding::warning("README lacks structured sections"));
            }
        } else {
            findings.push(Finding::missing("README.md not found"));
        }

        // Check for docs folder
        let has_docs = repo_data.tree.iter().any(|entry| {
            entry.path.starts_with("docs/") || entry.path.starts_with("documentation/")
        });

        if has_docs {
            score += 20.0;
            findings.push(Finding::positive("Documentation directory exists"));
        } else {
            findings.push(Finding::missing("No dedicated documentation directory"));
        }

        // Check for LICENSE
        if repo_data.has_license {
            score += 10.0;
            findings.push(Finding::positive("LICENSE file exists"));
        } else {
            findings.push(Finding::missing("LICENSE file not found"));
        }

        // Check for CONTRIBUTING.md
        let has_contributing = repo_data.tree.iter().any(|entry| {
            entry.path.eq_ignore_ascii_case("CONTRIBUTING.md")
        });

        if has_contributing {
            score += 10.0;
            findings.push(Finding::positive("CONTRIBUTING.md exists"));
        } else {
            findings.push(Finding::missing("CONTRIBUTING.md not found"));
        }

        let details = format!(
            "Found {} documentation elements. README quality: {}.",
            findings.iter().filter(|f| matches!(f.status, super::FindingStatus::Positive)).count(),
            if repo_data.readme_content.is_some() { "Present" } else { "Missing" }
        );

        Ok(AnalysisResult {
            score,
            details,
            findings,
        })
    }
}
