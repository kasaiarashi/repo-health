mod documentation;
mod tests;
mod ci_cd;
mod dependencies;
mod bus_factor;

pub use documentation::DocumentationAnalyzer;
pub use tests::TestsAnalyzer;
pub use ci_cd::CiCdAnalyzer;
pub use dependencies::DependenciesAnalyzer;
pub use bus_factor::BusFactorAnalyzer;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use crate::github::RepoData;
use crate::Result;

#[async_trait]
pub trait Analyzer: Send + Sync {
    fn name(&self) -> &str;
    async fn analyze(&self, repo_data: &RepoData) -> Result<AnalysisResult>;
    fn weight(&self) -> f64;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub score: f64,
    pub details: String,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub status: FindingStatus,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingStatus {
    Positive,
    Warning,
    Missing,
}

impl Finding {
    pub fn positive(message: impl Into<String>) -> Self {
        Self {
            status: FindingStatus::Positive,
            message: message.into(),
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            status: FindingStatus::Warning,
            message: message.into(),
        }
    }

    pub fn missing(message: impl Into<String>) -> Self {
        Self {
            status: FindingStatus::Missing,
            message: message.into(),
        }
    }
}
