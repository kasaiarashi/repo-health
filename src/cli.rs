use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "repo-health")]
#[command(version, about = "Analyze GitHub repository health", long_about = None)]
pub struct Cli {
    /// Repository in format "owner/repo" or full GitHub URL
    #[arg(value_name = "REPOSITORY")]
    pub repository: String,

    /// GitHub personal access token (or set GITHUB_TOKEN env var). Optional for public repos.
    #[arg(short, long, env = "GITHUB_TOKEN")]
    pub token: Option<String>,

    /// Output file path
    #[arg(short, long, default_value = "REPO_HEALTH.md")]
    pub output: String,

    /// Don't print to stdout, only write file
    #[arg(short, long)]
    pub quiet: bool,
}

/// Parse repository input from either "owner/repo" or full GitHub URL
pub fn parse_repo_input(input: &str) -> crate::Result<(String, String)> {
    use url::Url;
    use crate::error::RepoHealthError;

    if input.contains("github.com") {
        // Parse as URL
        let url = Url::parse(input)
            .map_err(|_| RepoHealthError::InvalidRepoFormat(input.to_string()))?;

        let segments: Vec<&str> = url
            .path_segments()
            .ok_or_else(|| RepoHealthError::InvalidRepoFormat(input.to_string()))?
            .collect();

        if segments.len() < 2 {
            return Err(RepoHealthError::InvalidRepoFormat(input.to_string()));
        }

        let owner = segments[0].to_string();
        let repo = segments[1].trim_end_matches(".git").to_string();

        Ok((owner, repo))
    } else {
        // Parse as owner/repo
        let parts: Vec<&str> = input.split('/').collect();

        if parts.len() != 2 {
            return Err(RepoHealthError::InvalidRepoFormat(
                "Expected format: owner/repo".to_string()
            ));
        }

        Ok((parts[0].to_string(), parts[1].to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_owner_repo() {
        let (owner, repo) = parse_repo_input("octocat/Hello-World").unwrap();
        assert_eq!(owner, "octocat");
        assert_eq!(repo, "Hello-World");
    }

    #[test]
    fn test_parse_github_url() {
        let (owner, repo) = parse_repo_input("https://github.com/octocat/Hello-World").unwrap();
        assert_eq!(owner, "octocat");
        assert_eq!(repo, "Hello-World");
    }

    #[test]
    fn test_parse_github_url_with_git() {
        let (owner, repo) = parse_repo_input("https://github.com/octocat/Hello-World.git").unwrap();
        assert_eq!(owner, "octocat");
        assert_eq!(repo, "Hello-World");
    }
}
