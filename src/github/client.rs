use octocrab::{Octocrab, models::Repository};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use crate::{Result, RepoHealthError};

#[derive(Debug, Clone)]
pub struct GitHubClient {
    octocrab: Octocrab,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeEntry {
    pub path: String,
    #[serde(rename = "type")]
    pub entry_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributorStats {
    pub author: Author,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub login: String,
}

#[derive(Debug, Clone)]
pub struct RepoData {
    pub repository: Repository,
    pub tree: Vec<TreeEntry>,
    pub contributors: Vec<ContributorStats>,
    pub readme_content: Option<String>,
    pub has_license: bool,
}

impl GitHubClient {
    pub fn new(token: String) -> Result<Self> {
        let octocrab = Octocrab::builder()
            .personal_token(token)
            .build()
            .map_err(|_| RepoHealthError::AuthenticationFailed)?;

        Ok(Self { octocrab })
    }

    pub async fn fetch_repository(&self, owner: &str, repo: &str) -> Result<Repository> {
        Ok(self.octocrab.repos(owner, repo).get().await?)
    }

    pub async fn fetch_tree(&self, owner: &str, repo: &str, branch: &str) -> Result<Vec<TreeEntry>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/git/trees/{}?recursive=1",
            owner, repo, branch
        );

        #[derive(Deserialize)]
        struct TreeResponse {
            tree: Vec<TreeEntry>,
        }

        let response: TreeResponse = self.octocrab
            .get(&url, None::<&()>)
            .await?;

        Ok(response.tree)
    }

    pub async fn fetch_contributors(&self, owner: &str, repo: &str) -> Result<Vec<ContributorStats>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/stats/contributors",
            owner, repo
        );

        // Handle 202 response (stats still being calculated)
        for attempt in 0..5 {
            match self.octocrab.get::<Vec<ContributorStats>, _, _>(&url, None::<&()>).await {
                Ok(contributors) => return Ok(contributors),
                Err(_) if attempt < 4 => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2u64.pow(attempt))).await;
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(Vec::new())
    }

    pub async fn fetch_readme(&self, owner: &str, repo: &str) -> Result<Option<String>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/readme",
            owner, repo
        );

        #[derive(Deserialize)]
        struct ReadmeResponse {
            content: String,
        }

        match self.octocrab.get::<ReadmeResponse, _, _>(&url, None::<&()>).await {
            Ok(response) => {
                // Base64 decode the content
                let decoded = String::from_utf8(
                    general_purpose::STANDARD.decode(&response.content.replace("\n", ""))
                        .map_err(|_| RepoHealthError::AnalysisFailed("Failed to decode README".to_string()))?
                ).map_err(|_| RepoHealthError::AnalysisFailed("Failed to decode README as UTF-8".to_string()))?;

                Ok(Some(decoded))
            }
            Err(_) => Ok(None),
        }
    }

    pub async fn fetch_license(&self, owner: &str, repo: &str) -> Result<bool> {
        let url = format!("https://api.github.com/repos/{}/{}/license", owner, repo);

        match self.octocrab.get::<serde_json::Value, _, _>(&url, None::<&()>).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub async fn fetch_workflows(&self, owner: &str, repo: &str) -> Result<Vec<String>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/workflows",
            owner, repo
        );

        #[derive(Deserialize)]
        struct WorkflowsResponse {
            workflows: Vec<Workflow>,
        }

        #[derive(Deserialize)]
        struct Workflow {
            name: String,
        }

        match self.octocrab.get::<WorkflowsResponse, _, _>(&url, None::<&()>).await {
            Ok(response) => Ok(response.workflows.into_iter().map(|w| w.name).collect()),
            Err(_) => Ok(Vec::new()),
        }
    }

    pub async fn fetch_all_data(&self, owner: &str, repo: &str) -> Result<RepoData> {
        let repository = self.fetch_repository(owner, repo).await?;
        let default_branch = repository.default_branch.as_deref().unwrap_or("main");

        let (tree, contributors, readme_content, has_license) = tokio::try_join!(
            self.fetch_tree(owner, repo, default_branch),
            self.fetch_contributors(owner, repo),
            self.fetch_readme(owner, repo),
            self.fetch_license(owner, repo),
        )?;

        Ok(RepoData {
            repository,
            tree,
            contributors,
            readme_content,
            has_license,
        })
    }
}
