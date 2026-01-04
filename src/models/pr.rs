use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::repo::Repository;
use super::user::{Link, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub state: PullRequestState,
    pub author: User,
    pub source: PullRequestEndpoint,
    pub destination: PullRequestEndpoint,
    pub merge_commit: Option<Commit>,
    pub close_source_branch: Option<bool>,
    pub closed_by: Option<User>,
    pub reason: Option<String>,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub reviewers: Option<Vec<User>>,
    pub participants: Option<Vec<Participant>>,
    pub links: Option<PullRequestLinks>,
    pub comment_count: Option<u32>,
    pub task_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PullRequestState {
    Open,
    Merged,
    Declined,
    Superseded,
}

impl std::fmt::Display for PullRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PullRequestState::Open => write!(f, "OPEN"),
            PullRequestState::Merged => write!(f, "MERGED"),
            PullRequestState::Declined => write!(f, "DECLINED"),
            PullRequestState::Superseded => write!(f, "SUPERSEDED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestEndpoint {
    pub branch: BranchInfo,
    pub commit: Option<CommitInfo>,
    pub repository: Option<Repository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub hash: String,
    pub message: Option<String>,
    pub author: Option<CommitAuthor>,
    pub date: Option<DateTime<Utc>>,
    pub links: Option<CommitLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitAuthor {
    pub raw: Option<String>,
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitLinks {
    pub html: Option<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub user: User,
    pub role: ParticipantRole,
    pub approved: bool,
    pub state: Option<ParticipantState>,
    pub participated_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ParticipantRole {
    Participant,
    Reviewer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParticipantState {
    Approved,
    ChangesRequested,
    #[serde(other)]
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestLinks {
    #[serde(rename = "self")]
    pub self_link: Option<Link>,
    pub html: Option<Link>,
    pub commits: Option<Link>,
    pub approve: Option<Link>,
    pub diff: Option<Link>,
    pub diffstat: Option<Link>,
    pub comments: Option<Link>,
    pub activity: Option<Link>,
    pub merge: Option<Link>,
    pub decline: Option<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePullRequestRequest {
    pub title: String,
    pub source: PullRequestBranchRef,
    pub destination: Option<PullRequestBranchRef>,
    pub description: Option<String>,
    pub close_source_branch: Option<bool>,
    pub reviewers: Option<Vec<UserRef>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestBranchRef {
    pub branch: BranchInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRef {
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergePullRequestRequest {
    #[serde(rename = "type")]
    pub merge_type: Option<String>,
    pub message: Option<String>,
    pub close_source_branch: Option<bool>,
    pub merge_strategy: Option<MergeStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeStrategy {
    MergeCommit,
    Squash,
    FastForward,
}

impl Default for MergePullRequestRequest {
    fn default() -> Self {
        Self {
            merge_type: Some("pullrequest".to_string()),
            message: None,
            close_source_branch: Some(true),
            merge_strategy: Some(MergeStrategy::MergeCommit),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestComment {
    pub id: u64,
    pub content: CommentContent,
    pub user: User,
    pub created_on: DateTime<Utc>,
    pub updated_on: Option<DateTime<Utc>>,
    pub deleted: Option<bool>,
    pub inline: Option<InlineComment>,
    pub parent: Option<CommentRef>,
    pub links: Option<CommentLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentContent {
    pub raw: String,
    pub markup: Option<String>,
    pub html: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineComment {
    pub from: Option<u32>,
    pub to: Option<u32>,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentRef {
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentLinks {
    #[serde(rename = "self")]
    pub self_link: Option<Link>,
    pub html: Option<Link>,
}
