use anyhow::Result;

use super::BitbucketClient;
use crate::models::{Paginated, Pipeline, PipelineStep, TriggerPipelineRequest};

impl BitbucketClient {
    /// List pipelines for a repository
    pub async fn list_pipelines(
        &self,
        workspace: &str,
        repo_slug: &str,
        page: Option<u32>,
        pagelen: Option<u32>,
    ) -> Result<Paginated<Pipeline>> {
        let mut query = Vec::new();

        // Sort by created_on descending to get most recent first
        query.push(("sort", "-created_on".to_string()));

        if let Some(p) = page {
            query.push(("page", p.to_string()));
        }
        if let Some(len) = pagelen {
            query.push(("pagelen", len.to_string()));
        }

        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let path = format!("/repositories/{}/{}/pipelines", workspace, repo_slug);
        self.get_with_query(&path, &query_refs).await
    }

    /// Get a specific pipeline
    pub async fn get_pipeline(
        &self,
        workspace: &str,
        repo_slug: &str,
        pipeline_uuid: &str,
    ) -> Result<Pipeline> {
        let path = format!(
            "/repositories/{}/{}/pipelines/{}",
            workspace, repo_slug, pipeline_uuid
        );
        self.get(&path).await
    }

    /// Trigger a new pipeline
    pub async fn trigger_pipeline(
        &self,
        workspace: &str,
        repo_slug: &str,
        request: &TriggerPipelineRequest,
    ) -> Result<Pipeline> {
        let path = format!("/repositories/{}/{}/pipelines", workspace, repo_slug);
        self.post(&path, request).await
    }

    /// Stop a running pipeline
    pub async fn stop_pipeline(
        &self,
        workspace: &str,
        repo_slug: &str,
        pipeline_uuid: &str,
    ) -> Result<()> {
        let path = format!(
            "/repositories/{}/{}/pipelines/{}/stopPipeline",
            workspace, repo_slug, pipeline_uuid
        );
        self.post_no_response(&path, &serde_json::json!({})).await
    }

    /// List steps for a pipeline
    pub async fn list_pipeline_steps(
        &self,
        workspace: &str,
        repo_slug: &str,
        pipeline_uuid: &str,
    ) -> Result<Paginated<PipelineStep>> {
        let path = format!(
            "/repositories/{}/{}/pipelines/{}/steps",
            workspace, repo_slug, pipeline_uuid
        );
        self.get(&path).await
    }

    /// Get a specific pipeline step
    pub async fn get_pipeline_step(
        &self,
        workspace: &str,
        repo_slug: &str,
        pipeline_uuid: &str,
        step_uuid: &str,
    ) -> Result<PipelineStep> {
        let path = format!(
            "/repositories/{}/{}/pipelines/{}/steps/{}",
            workspace, repo_slug, pipeline_uuid, step_uuid
        );
        self.get(&path).await
    }

    /// Get pipeline step log
    pub async fn get_step_log(
        &self,
        workspace: &str,
        repo_slug: &str,
        pipeline_uuid: &str,
        step_uuid: &str,
    ) -> Result<String> {
        let path = format!(
            "/repositories/{}/{}/pipelines/{}/steps/{}/log",
            workspace, repo_slug, pipeline_uuid, step_uuid
        );

        let response = reqwest::Client::new()
            .get(self.url(&path))
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            anyhow::bail!("Failed to get step log: {}", response.status())
        }
    }

    /// Get pipeline by build number
    pub async fn get_pipeline_by_build_number(
        &self,
        workspace: &str,
        repo_slug: &str,
        build_number: u64,
    ) -> Result<Pipeline> {
        // Search for the pipeline with the given build number
        let pipelines = self
            .list_pipelines(workspace, repo_slug, Some(1), Some(100))
            .await?;

        pipelines
            .values
            .into_iter()
            .find(|p| p.build_number == build_number)
            .ok_or_else(|| anyhow::anyhow!("Pipeline #{} not found", build_number))
    }
}
