use anyhow::Result;

use super::BitbucketClient;
use crate::models::{
    CreateIssueCommentRequest, CreateIssueRequest, Issue, IssueComment, IssueState, Paginated,
};

impl BitbucketClient {
    /// List issues for a repository
    pub async fn list_issues(
        &self,
        workspace: &str,
        repo_slug: &str,
        state: Option<IssueState>,
        page: Option<u32>,
        pagelen: Option<u32>,
    ) -> Result<Paginated<Issue>> {
        let mut query = Vec::new();

        if let Some(s) = state {
            query.push(("state", s.to_string()));
        }
        if let Some(p) = page {
            query.push(("page", p.to_string()));
        }
        if let Some(len) = pagelen {
            query.push(("pagelen", len.to_string()));
        }

        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let path = format!("/repositories/{}/{}/issues", workspace, repo_slug);
        self.get_with_query(&path, &query_refs).await
    }

    /// Get a specific issue
    pub async fn get_issue(
        &self,
        workspace: &str,
        repo_slug: &str,
        issue_id: u64,
    ) -> Result<Issue> {
        let path = format!(
            "/repositories/{}/{}/issues/{}",
            workspace, repo_slug, issue_id
        );
        self.get(&path).await
    }

    /// Create a new issue
    pub async fn create_issue(
        &self,
        workspace: &str,
        repo_slug: &str,
        request: &CreateIssueRequest,
    ) -> Result<Issue> {
        let path = format!("/repositories/{}/{}/issues", workspace, repo_slug);
        self.post(&path, request).await
    }

    /// Update an issue
    pub async fn update_issue(
        &self,
        workspace: &str,
        repo_slug: &str,
        issue_id: u64,
        title: Option<&str>,
        content: Option<&str>,
        state: Option<IssueState>,
    ) -> Result<Issue> {
        #[derive(serde::Serialize)]
        struct UpdateRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<ContentRequest>,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<IssueState>,
        }

        #[derive(serde::Serialize)]
        struct ContentRequest {
            raw: String,
        }

        let request = UpdateRequest {
            title: title.map(|t| t.to_string()),
            content: content.map(|c| ContentRequest {
                raw: c.to_string(),
            }),
            state,
        };

        let path = format!(
            "/repositories/{}/{}/issues/{}",
            workspace, repo_slug, issue_id
        );
        self.put(&path, &request).await
    }

    /// Delete an issue
    pub async fn delete_issue(
        &self,
        workspace: &str,
        repo_slug: &str,
        issue_id: u64,
    ) -> Result<()> {
        let path = format!(
            "/repositories/{}/{}/issues/{}",
            workspace, repo_slug, issue_id
        );
        self.delete(&path).await
    }

    /// List comments on an issue
    pub async fn list_issue_comments(
        &self,
        workspace: &str,
        repo_slug: &str,
        issue_id: u64,
    ) -> Result<Paginated<IssueComment>> {
        let path = format!(
            "/repositories/{}/{}/issues/{}/comments",
            workspace, repo_slug, issue_id
        );
        self.get(&path).await
    }

    /// Add a comment to an issue
    pub async fn add_issue_comment(
        &self,
        workspace: &str,
        repo_slug: &str,
        issue_id: u64,
        content: &str,
    ) -> Result<IssueComment> {
        let request = CreateIssueCommentRequest {
            content: crate::models::IssueContentRequest {
                raw: content.to_string(),
            },
        };

        let path = format!(
            "/repositories/{}/{}/issues/{}/comments",
            workspace, repo_slug, issue_id
        );
        self.post(&path, &request).await
    }

    /// Vote for an issue
    pub async fn vote_issue(
        &self,
        workspace: &str,
        repo_slug: &str,
        issue_id: u64,
    ) -> Result<()> {
        let path = format!(
            "/repositories/{}/{}/issues/{}/vote",
            workspace, repo_slug, issue_id
        );
        self.put::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Remove vote from an issue
    pub async fn unvote_issue(
        &self,
        workspace: &str,
        repo_slug: &str,
        issue_id: u64,
    ) -> Result<()> {
        let path = format!(
            "/repositories/{}/{}/issues/{}/vote",
            workspace, repo_slug, issue_id
        );
        self.delete(&path).await
    }

    /// Watch an issue
    pub async fn watch_issue(
        &self,
        workspace: &str,
        repo_slug: &str,
        issue_id: u64,
    ) -> Result<()> {
        let path = format!(
            "/repositories/{}/{}/issues/{}/watch",
            workspace, repo_slug, issue_id
        );
        self.put::<serde_json::Value, _>(&path, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Unwatch an issue
    pub async fn unwatch_issue(
        &self,
        workspace: &str,
        repo_slug: &str,
        issue_id: u64,
    ) -> Result<()> {
        let path = format!(
            "/repositories/{}/{}/issues/{}/watch",
            workspace, repo_slug, issue_id
        );
        self.delete(&path).await
    }
}
