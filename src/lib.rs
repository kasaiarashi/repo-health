pub mod cli;
pub mod error;
pub mod github;
pub mod analyzers;
pub mod scoring;
pub mod output;

pub use error::RepoHealthError;
pub type Result<T> = std::result::Result<T, RepoHealthError>;
