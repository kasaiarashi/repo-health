use async_trait::async_trait;
use crate::github::RepoData;
use crate::Result;
use super::{Analyzer, AnalysisResult, Finding};

pub struct BusFactorAnalyzer;

impl BusFactorAnalyzer {
    fn calculate_bus_factor(&self, contributors: &[crate::github::ContributorStats]) -> (usize, Vec<(String, i64, f64)>) {
        if contributors.is_empty() {
            return (0, Vec::new());
        }

        // Calculate total commits
        let total_commits: i64 = contributors.iter().map(|c| c.total).sum();

        if total_commits == 0 {
            return (0, Vec::new());
        }

        // Sort contributors by commit count (descending)
        let mut sorted_contributors: Vec<_> = contributors.iter()
            .map(|c| (c.author.login.clone(), c.total))
            .collect();
        sorted_contributors.sort_by(|a, b| b.1.cmp(&a.1));

        // Calculate bus factor: minimum number of contributors accounting for 50% of commits
        let target_commits = total_commits / 2;
        let mut cumulative_commits = 0i64;
        let mut bus_factor = 0usize;

        let mut top_contributors = Vec::new();

        for (login, commits) in &sorted_contributors {
            cumulative_commits += commits;
            bus_factor += 1;

            let percentage = (*commits as f64 / total_commits as f64) * 100.0;
            if top_contributors.len() < 5 {
                top_contributors.push((login.clone(), *commits, percentage));
            }

            if cumulative_commits >= target_commits {
                break;
            }
        }

        (bus_factor, top_contributors)
    }

    fn score_bus_factor(&self, bus_factor: usize) -> f64 {
        match bus_factor {
            0 => 0.0,
            1 => 10.0,
            2 => 40.0,
            3..=4 => 70.0,
            _ => 100.0,
        }
    }
}

#[async_trait]
impl Analyzer for BusFactorAnalyzer {
    fn name(&self) -> &str {
        "Bus Factor"
    }

    fn weight(&self) -> f64 {
        0.15
    }

    async fn analyze(&self, repo_data: &RepoData) -> Result<AnalysisResult> {
        let mut findings = Vec::new();

        if repo_data.contributors.is_empty() {
            findings.push(Finding::warning("No contributor data available"));
            return Ok(AnalysisResult {
                score: 50.0, // Neutral score when data unavailable
                details: "Unable to calculate bus factor - no contributor data".to_string(),
                findings,
            });
        }

        let (bus_factor, top_contributors) = self.calculate_bus_factor(&repo_data.contributors);
        let score = self.score_bus_factor(bus_factor);

        findings.push(Finding::positive(format!(
            "Bus factor: {} (minimum contributors accounting for 50% of commits)",
            bus_factor
        )));

        // Add information about top contributors
        if !top_contributors.is_empty() {
            findings.push(Finding::positive(format!(
                "Total contributors: {}",
                repo_data.contributors.len()
            )));

            for (idx, (login, commits, percentage)) in top_contributors.iter().enumerate() {
                let status = if idx == 0 && *percentage > 70.0 {
                    Finding::warning(format!(
                        "{}: {} commits ({:.1}%) - High concentration of ownership",
                        login, commits, percentage
                    ))
                } else {
                    Finding::positive(format!(
                        "{}: {} commits ({:.1}%)",
                        login, commits, percentage
                    ))
                };
                findings.push(status);
            }
        }

        let details = match bus_factor {
            0 => "No commit data available".to_string(),
            1 => "Critical: Single person controls >50% of commits".to_string(),
            2 => "Low: Two people control >50% of commits".to_string(),
            3..=4 => "Moderate: Small team of 3-4 core contributors".to_string(),
            _ => "Healthy: Well-distributed contributions".to_string(),
        };

        Ok(AnalysisResult {
            score,
            details,
            findings,
        })
    }
}
