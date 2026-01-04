use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::repo::Repository;
use super::user::{Link, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub uuid: String,
    pub build_number: u64,
    pub creator: Option<User>,
    pub repository: Option<Repository>,
    pub target: PipelineTarget,
    pub trigger: Option<PipelineTrigger>,
    pub state: PipelineState,
    pub created_on: DateTime<Utc>,
    pub completed_on: Option<DateTime<Utc>>,
    pub build_seconds_used: Option<u64>,
    pub links: Option<PipelineLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineTarget {
    #[serde(rename = "type")]
    pub target_type: String,
    pub ref_type: Option<String>,
    pub ref_name: Option<String>,
    pub selector: Option<PipelineSelector>,
    pub commit: Option<PipelineCommit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineSelector {
    #[serde(rename = "type")]
    pub selector_type: String,
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineCommit {
    pub hash: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineTrigger {
    #[serde(rename = "type")]
    pub trigger_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineState {
    pub name: PipelineStateName,
    #[serde(rename = "type")]
    pub state_type: String,
    pub result: Option<PipelineResult>,
    pub stage: Option<PipelineStage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PipelineStateName {
    Pending,
    Building,
    Completed,
    Halted,
    Paused,
}

impl std::fmt::Display for PipelineStateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineStateName::Pending => write!(f, "PENDING"),
            PipelineStateName::Building => write!(f, "BUILDING"),
            PipelineStateName::Completed => write!(f, "COMPLETED"),
            PipelineStateName::Halted => write!(f, "HALTED"),
            PipelineStateName::Paused => write!(f, "PAUSED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    pub name: PipelineResultName,
    #[serde(rename = "type")]
    pub result_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PipelineResultName {
    Successful,
    Failed,
    Error,
    Stopped,
    Expired,
}

impl std::fmt::Display for PipelineResultName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineResultName::Successful => write!(f, "SUCCESSFUL"),
            PipelineResultName::Failed => write!(f, "FAILED"),
            PipelineResultName::Error => write!(f, "ERROR"),
            PipelineResultName::Stopped => write!(f, "STOPPED"),
            PipelineResultName::Expired => write!(f, "EXPIRED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub name: String,
    #[serde(rename = "type")]
    pub stage_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineLinks {
    #[serde(rename = "self")]
    pub self_link: Option<Link>,
    pub steps: Option<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStep {
    pub uuid: String,
    pub name: Option<String>,
    pub started_on: Option<DateTime<Utc>>,
    pub completed_on: Option<DateTime<Utc>>,
    pub state: Option<PipelineStepState>,
    pub image: Option<PipelineImage>,
    pub setup_commands: Option<Vec<PipelineCommand>>,
    pub script_commands: Option<Vec<PipelineCommand>>,
    pub links: Option<PipelineStepLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStepState {
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: String,
    pub result: Option<PipelineStepResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStepResult {
    pub name: String,
    #[serde(rename = "type")]
    pub result_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineImage {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineCommand {
    pub name: String,
    pub command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStepLinks {
    #[serde(rename = "self")]
    pub self_link: Option<Link>,
    pub log: Option<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerPipelineRequest {
    pub target: TriggerPipelineTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerPipelineTarget {
    #[serde(rename = "type")]
    pub target_type: String,
    pub ref_type: String,
    pub ref_name: String,
    pub selector: Option<TriggerPipelineSelector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerPipelineSelector {
    #[serde(rename = "type")]
    pub selector_type: String,
    pub pattern: String,
}

impl TriggerPipelineRequest {
    pub fn for_branch(branch: &str) -> Self {
        Self {
            target: TriggerPipelineTarget {
                target_type: "pipeline_ref_target".to_string(),
                ref_type: "branch".to_string(),
                ref_name: branch.to_string(),
                selector: None,
            },
        }
    }

    pub fn for_branch_with_pipeline(branch: &str, pipeline: &str) -> Self {
        Self {
            target: TriggerPipelineTarget {
                target_type: "pipeline_ref_target".to_string(),
                ref_type: "branch".to_string(),
                ref_name: branch.to_string(),
                selector: Some(TriggerPipelineSelector {
                    selector_type: "custom".to_string(),
                    pattern: pipeline.to_string(),
                }),
            },
        }
    }
}
