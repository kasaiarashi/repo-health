use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoHealthError {
    #[error("GitHub API error: {0}")]
    GitHubApi(#[from] octocrab::Error),

    #[error("Invalid repository format: {0}")]
    InvalidRepoFormat(String),

    #[error("Authentication failed: missing or invalid GITHUB_TOKEN")]
    AuthenticationFailed,

    #[error("Rate limit exceeded. Resets at {reset_time}")]
    RateLimitExceeded { reset_time: String },

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Template error: {0}")]
    Template(#[from] handlebars::RenderError),
}
