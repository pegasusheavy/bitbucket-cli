use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

use bitbucket_cli::auth::Credential;
use bitbucket_cli::config::Config;
use bitbucket_cli::models::*;

// Sample JSON data for benchmarking deserialization
const REPOSITORY_JSON: &str = r#"{
    "uuid": "{12345678-1234-1234-1234-123456789abc}",
    "name": "my-repo",
    "full_name": "workspace/my-repo",
    "slug": "my-repo",
    "description": "A sample repository for benchmarking",
    "is_private": true,
    "scm": "git",
    "created_on": "2024-01-15T10:30:00.000000+00:00",
    "updated_on": "2024-06-20T15:45:00.000000+00:00",
    "size": 1024000,
    "language": "Rust",
    "has_issues": true,
    "has_wiki": false,
    "fork_policy": "no_public_forks"
}"#;

const PULL_REQUEST_JSON: &str = r#"{
    "id": 42,
    "title": "Add new feature implementation",
    "description": "This PR adds a comprehensive new feature with tests and documentation",
    "state": "OPEN",
    "author": {
        "uuid": "{user-uuid-1234}",
        "display_name": "John Developer",
        "type": "user"
    },
    "source": {
        "branch": {"name": "feature/new-feature"},
        "commit": {"hash": "abc123def456"}
    },
    "destination": {
        "branch": {"name": "main"},
        "commit": {"hash": "789xyz000111"}
    },
    "created_on": "2024-06-01T09:00:00.000000+00:00",
    "updated_on": "2024-06-15T14:30:00.000000+00:00",
    "comment_count": 5,
    "task_count": 2
}"#;

const ISSUE_JSON: &str = r#"{
    "id": 123,
    "title": "Bug: Application crashes on startup",
    "content": {
        "raw": "When starting the application with certain flags, it crashes immediately.",
        "markup": "markdown"
    },
    "state": "open",
    "kind": "bug",
    "priority": "critical",
    "votes": 15,
    "watches": 8,
    "created_on": "2024-05-10T08:00:00.000000+00:00",
    "updated_on": "2024-06-18T16:00:00.000000+00:00"
}"#;

const PAGINATED_REPOS_JSON: &str = r#"{
    "size": 100,
    "page": 1,
    "pagelen": 10,
    "next": "https://api.bitbucket.org/2.0/repositories?page=2",
    "values": [
        {"uuid": "{repo-1}", "name": "repo-1", "full_name": "ws/repo-1", "slug": "repo-1", "is_private": true, "scm": "git"},
        {"uuid": "{repo-2}", "name": "repo-2", "full_name": "ws/repo-2", "slug": "repo-2", "is_private": false, "scm": "git"},
        {"uuid": "{repo-3}", "name": "repo-3", "full_name": "ws/repo-3", "slug": "repo-3", "is_private": true, "scm": "git"},
        {"uuid": "{repo-4}", "name": "repo-4", "full_name": "ws/repo-4", "slug": "repo-4", "is_private": true, "scm": "git"},
        {"uuid": "{repo-5}", "name": "repo-5", "full_name": "ws/repo-5", "slug": "repo-5", "is_private": false, "scm": "git"}
    ]
}"#;

const CONFIG_TOML: &str = r#"
[auth]
username = "testuser"
default_workspace = "myworkspace"

[defaults]
workspace = "myworkspace"
repository = "myrepo"
branch = "main"

[display]
color = true
pager = true
date_format = "%Y-%m-%d %H:%M"
"#;

fn create_sample_repository() -> Repository {
    Repository {
        uuid: "{12345678-1234-1234-1234-123456789abc}".to_string(),
        name: "my-repo".to_string(),
        full_name: "workspace/my-repo".to_string(),
        slug: "my-repo".to_string(),
        description: Some("A sample repository".to_string()),
        is_private: true,
        scm: "git".to_string(),
        owner: None,
        workspace: None,
        project: None,
        created_on: Some(chrono::Utc::now()),
        updated_on: Some(chrono::Utc::now()),
        size: Some(1024000),
        language: Some("Rust".to_string()),
        has_issues: Some(true),
        has_wiki: Some(false),
        fork_policy: Some("no_public_forks".to_string()),
        mainbranch: Some(Branch {
            name: "main".to_string(),
            branch_type: Some("branch".to_string()),
        }),
        links: None,
    }
}

fn create_sample_issue() -> Issue {
    Issue {
        id: 123,
        title: "Bug: Application crashes on startup".to_string(),
        content: Some(IssueContent {
            raw: Some("Description of the issue".to_string()),
            markup: Some("markdown".to_string()),
            html: None,
        }),
        reporter: Some(User {
            uuid: "{user-uuid}".to_string(),
            username: Some("reporter".to_string()),
            display_name: "Bug Reporter".to_string(),
            account_id: Some("account123".to_string()),
            user_type: "user".to_string(),
            links: None,
        }),
        assignee: None,
        state: IssueState::Open,
        kind: IssueKind::Bug,
        priority: IssuePriority::Critical,
        milestone: None,
        component: None,
        version: None,
        votes: Some(15),
        watches: Some(8),
        created_on: chrono::Utc::now(),
        updated_on: Some(chrono::Utc::now()),
        edited_on: None,
        links: None,
    }
}

fn create_sample_credential() -> Credential {
    Credential::AppPassword {
        username: "testuser".to_string(),
        app_password: "super_secret_app_password_12345".to_string(),
    }
}

// Benchmark JSON deserialization
fn bench_json_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_deserialization");

    group.throughput(Throughput::Bytes(REPOSITORY_JSON.len() as u64));
    group.bench_function("repository", |b| {
        b.iter(|| {
            let repo: Repository = serde_json::from_str(black_box(REPOSITORY_JSON)).unwrap();
            black_box(repo)
        })
    });

    group.throughput(Throughput::Bytes(PULL_REQUEST_JSON.len() as u64));
    group.bench_function("pull_request", |b| {
        b.iter(|| {
            let pr: PullRequest = serde_json::from_str(black_box(PULL_REQUEST_JSON)).unwrap();
            black_box(pr)
        })
    });

    group.throughput(Throughput::Bytes(ISSUE_JSON.len() as u64));
    group.bench_function("issue", |b| {
        b.iter(|| {
            let issue: Issue = serde_json::from_str(black_box(ISSUE_JSON)).unwrap();
            black_box(issue)
        })
    });

    group.throughput(Throughput::Bytes(PAGINATED_REPOS_JSON.len() as u64));
    group.bench_function("paginated_repositories", |b| {
        b.iter(|| {
            let paginated: Paginated<Repository> =
                serde_json::from_str(black_box(PAGINATED_REPOS_JSON)).unwrap();
            black_box(paginated)
        })
    });

    group.finish();
}

// Benchmark JSON serialization
fn bench_json_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_serialization");

    let repo = create_sample_repository();
    group.bench_function("repository", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&repo)).unwrap();
            black_box(json)
        })
    });

    let issue = create_sample_issue();
    group.bench_function("issue", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&issue)).unwrap();
            black_box(json)
        })
    });

    group.finish();
}

// Benchmark auth header generation
fn bench_auth_header(c: &mut Criterion) {
    let mut group = c.benchmark_group("auth");

    let app_password = Credential::AppPassword {
        username: "testuser".to_string(),
        app_password: "app_password_secret_value".to_string(),
    };

    let oauth = Credential::OAuth {
        access_token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ".to_string(),
        refresh_token: Some("refresh_token_value".to_string()),
        expires_at: Some(chrono::Utc::now().timestamp() + 3600),
    };

    group.bench_function("app_password_header", |b| {
        b.iter(|| {
            let header = black_box(&app_password).auth_header();
            black_box(header)
        })
    });

    group.bench_function("oauth_header", |b| {
        b.iter(|| {
            let header = black_box(&oauth).auth_header();
            black_box(header)
        })
    });

    group.bench_function("needs_refresh_check", |b| {
        b.iter(|| {
            let needs = black_box(&oauth).needs_refresh();
            black_box(needs)
        })
    });

    group.finish();
}

// Benchmark config parsing
fn bench_config_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("config");

    group.throughput(Throughput::Bytes(CONFIG_TOML.len() as u64));
    group.bench_function("toml_parse", |b| {
        b.iter(|| {
            let config: Config = toml::from_str(black_box(CONFIG_TOML)).unwrap();
            black_box(config)
        })
    });

    let config = Config::default();
    group.bench_function("toml_serialize", |b| {
        b.iter(|| {
            let toml_str = toml::to_string(black_box(&config)).unwrap();
            black_box(toml_str)
        })
    });

    group.finish();
}

// Benchmark credential serialization (for keyring storage)
fn bench_credential_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("credential_serialization");

    let credential = create_sample_credential();
    let credential_json = serde_json::to_string(&credential).unwrap();

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&credential)).unwrap();
            black_box(json)
        })
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let cred: Credential = serde_json::from_str(black_box(&credential_json)).unwrap();
            black_box(cred)
        })
    });

    group.finish();
}

// Benchmark varying payload sizes
fn bench_payload_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("payload_sizes");

    for size in [1, 10, 50, 100].iter() {
        let repos: Vec<Repository> = (0..*size)
            .map(|i| Repository {
                uuid: format!("{{uuid-{}}}", i),
                name: format!("repo-{}", i),
                full_name: format!("workspace/repo-{}", i),
                slug: format!("repo-{}", i),
                description: Some(format!("Description for repo {}", i)),
                is_private: i % 2 == 0,
                scm: "git".to_string(),
                owner: None,
                workspace: None,
                project: None,
                created_on: Some(chrono::Utc::now()),
                updated_on: Some(chrono::Utc::now()),
                size: Some(1024 * (i as u64 + 1)),
                language: Some("Rust".to_string()),
                has_issues: Some(true),
                has_wiki: Some(false),
                fork_policy: None,
                mainbranch: None,
                links: None,
            })
            .collect();

        let paginated = Paginated {
            size: Some(*size as u32),
            page: Some(1),
            pagelen: Some(*size as u32),
            next: None,
            previous: None,
            values: repos,
        };

        let json = serde_json::to_string(&paginated).unwrap();

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("deserialize_repos", size),
            &json,
            |b, json| {
                b.iter(|| {
                    let p: Paginated<Repository> = serde_json::from_str(black_box(json)).unwrap();
                    black_box(p)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("serialize_repos", size),
            &paginated,
            |b, paginated| {
                b.iter(|| {
                    let json = serde_json::to_string(black_box(paginated)).unwrap();
                    black_box(json)
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_json_deserialization,
    bench_json_serialization,
    bench_auth_header,
    bench_config_parsing,
    bench_credential_serialization,
    bench_payload_sizes,
);

criterion_main!(benches);
