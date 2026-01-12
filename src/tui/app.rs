use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use super::event::{Event, EventHandler};
use super::ui;
use super::views::{View, ViewState};
use crate::api::BitbucketClient;
use crate::models::{Issue, Pipeline, PullRequest, Repository};

/// Application state
pub struct App {
    /// Is the application running
    pub running: bool,
    /// Current view
    pub current_view: View,
    /// View-specific state
    pub view_state: ViewState,
    /// API client
    pub client: Option<BitbucketClient>,
    /// Current workspace
    pub workspace: Option<String>,
    /// Status message
    pub status: Option<String>,
    /// Is loading data
    pub loading: bool,
    /// Error message
    pub error: Option<String>,

    // Data
    pub repositories: Vec<Repository>,
    pub pull_requests: Vec<PullRequest>,
    pub issues: Vec<Issue>,
    pub pipelines: Vec<Pipeline>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            current_view: View::Dashboard,
            view_state: ViewState::default(),
            client: None,
            workspace: None,
            status: None,
            loading: false,
            error: None,
            repositories: Vec::new(),
            pull_requests: Vec::new(),
            issues: Vec::new(),
            pipelines: Vec::new(),
        }
    }

    /// Initialize the application with API client
    pub fn with_client(mut self, client: BitbucketClient) -> Self {
        self.client = Some(client);
        self
    }

    /// Set the workspace
    pub fn with_workspace(mut self, workspace: String) -> Self {
        self.workspace = Some(workspace);
        self
    }

    /// Set status message
    pub fn set_status(&mut self, message: &str) {
        self.status = Some(message.to_string());
    }

    /// Clear status message
    pub fn clear_status(&mut self) {
        self.status = None;
    }

    /// Set error message
    pub fn set_error(&mut self, message: &str) {
        self.error = Some(message.to_string());
    }

    /// Clear error
    pub fn clear_error(&mut self) {
        self.error = None;
    }

    /// Switch to a different view
    pub fn switch_view(&mut self, view: View) {
        self.current_view = view;
        self.view_state.selected_index = 0;
        self.clear_error();
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        // Global keys
        match key.code {
            KeyCode::Char('q') => {
                self.running = false;
                return;
            }
            KeyCode::Char('1') => {
                self.switch_view(View::Dashboard);
                return;
            }
            KeyCode::Char('2') => {
                self.switch_view(View::Repositories);
                return;
            }
            KeyCode::Char('3') => {
                self.switch_view(View::PullRequests);
                return;
            }
            KeyCode::Char('4') => {
                self.switch_view(View::Issues);
                return;
            }
            KeyCode::Char('5') => {
                self.switch_view(View::Pipelines);
                return;
            }
            KeyCode::Esc => {
                self.clear_error();
                return;
            }
            _ => {}
        }

        // View-specific keys
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.view_state.previous();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max = match self.current_view {
                    View::Dashboard => 4,
                    View::Repositories => self.repositories.len(),
                    View::PullRequests => self.pull_requests.len(),
                    View::Issues => self.issues.len(),
                    View::Pipelines => self.pipelines.len(),
                };
                self.view_state.next(max);
            }
            KeyCode::Enter => {
                self.handle_select();
            }
            KeyCode::Char('r') => {
                // Refresh will be handled in main loop
            }
            _ => {}
        }
    }

    /// Handle selection
    fn handle_select(&mut self) {
        match self.current_view {
            View::Dashboard => {
                // Navigate to selected section
                match self.view_state.selected_index {
                    0 => self.switch_view(View::Repositories),
                    1 => self.switch_view(View::PullRequests),
                    2 => self.switch_view(View::Issues),
                    3 => self.switch_view(View::Pipelines),
                    _ => {}
                }
            }
            View::Repositories => {
                if let Some(repo) = self.repositories.get(self.view_state.selected_index) {
                    self.set_status(&format!("Selected: {}", repo.full_name));
                }
            }
            View::PullRequests => {
                if let Some(pr) = self.pull_requests.get(self.view_state.selected_index) {
                    self.set_status(&format!("Selected PR #{}: {}", pr.id, pr.title));
                }
            }
            View::Issues => {
                if let Some(issue) = self.issues.get(self.view_state.selected_index) {
                    self.set_status(&format!("Selected Issue #{}: {}", issue.id, issue.title));
                }
            }
            View::Pipelines => {
                if let Some(pipeline) = self.pipelines.get(self.view_state.selected_index) {
                    self.set_status(&format!("Selected Pipeline #{}", pipeline.build_number));
                }
            }
        }
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Load repositories
    pub async fn load_repositories(&mut self) -> Result<()> {
        if let (Some(client), Some(workspace)) = (&self.client, &self.workspace) {
            self.loading = true;
            match client.list_repositories(workspace, None, Some(50)).await {
                Ok(result) => {
                    self.repositories = result.values;
                    self.clear_error();
                }
                Err(e) => {
                    self.set_error(&format!("Failed to load repositories: {}", e));
                }
            }
            self.loading = false;
        } else {
            self.set_error("No workspace configured");
        }
        Ok(())
    }

    /// Load pull requests for the current workspace
    pub async fn load_pull_requests(&mut self) -> Result<()> {
        if let (Some(client), Some(workspace)) = (&self.client, &self.workspace) {
            self.loading = true;
            self.pull_requests.clear();
            
            // Load PRs from all repositories
            if let Ok(repos) = client.list_repositories(workspace, None, Some(50)).await {
                for repo in repos.values {
                    let repo_slug = repo.slug.as_deref().unwrap_or(&repo.name);
                    if let Ok(prs) = client.list_pull_requests(workspace, repo_slug, None, None, Some(10)).await {
                        self.pull_requests.extend(prs.values);
                    }
                }
            }
            
            self.clear_error();
            self.loading = false;
        } else {
            self.set_error("No workspace configured");
        }
        Ok(())
    }

    /// Load issues for the current workspace
    pub async fn load_issues(&mut self) -> Result<()> {
        if let (Some(client), Some(workspace)) = (&self.client, &self.workspace) {
            self.loading = true;
            self.issues.clear();
            
            // Load issues from all repositories
            if let Ok(repos) = client.list_repositories(workspace, None, Some(50)).await {
                for repo in repos.values {
                    let repo_slug = repo.slug.as_deref().unwrap_or(&repo.name);
                    if let Ok(issues) = client.list_issues(workspace, repo_slug, None, None, Some(10)).await {
                        self.issues.extend(issues.values);
                    }
                }
            }
            
            self.clear_error();
            self.loading = false;
        } else {
            self.set_error("No workspace configured");
        }
        Ok(())
    }

    /// Load pipelines for the current workspace
    pub async fn load_pipelines(&mut self) -> Result<()> {
        if let (Some(client), Some(workspace)) = (&self.client, &self.workspace) {
            self.loading = true;
            self.pipelines.clear();
            
            // Load pipelines from all repositories
            if let Ok(repos) = client.list_repositories(workspace, None, Some(50)).await {
                for repo in repos.values {
                    let repo_slug = repo.slug.as_deref().unwrap_or(&repo.name);
                    if let Ok(pipelines) = client.list_pipelines(workspace, repo_slug, None, Some(10)).await {
                        self.pipelines.extend(pipelines.values);
                    }
                }
            }
            
            self.clear_error();
            self.loading = false;
        } else {
            self.set_error("No workspace configured");
        }
        Ok(())
    }

    /// Load all data
    pub async fn load_all_data(&mut self) -> Result<()> {
        self.load_repositories().await?;
        self.load_pull_requests().await?;
        self.load_issues().await?;
        self.load_pipelines().await?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Run the TUI application
pub async fn run_tui(workspace: Option<String>) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

    // Try to get API client
    match BitbucketClient::from_stored() {
        Ok(client) => {
            app = app.with_client(client);
            if let Some(ws) = workspace {
                app = app.with_workspace(ws);
            } else {
                app.set_error("No workspace specified. Use: bitbucket tui --workspace <workspace>");
            }
        }
        Err(e) => {
            app.set_error(&format!("Not authenticated: {}", e));
        }
    }

    // Load initial data if we have a workspace
    if app.workspace.is_some() && app.client.is_some() {
        app.set_status("Loading data...");
        terminal.draw(|f| ui::draw(f, &app))?;
        
        if let Err(e) = app.load_repositories().await {
            app.set_error(&format!("Failed to load data: {}", e));
        } else {
            app.set_status("Data loaded. Press 'r' to refresh.");
        }
    }

    // Create event handler
    let event_handler = EventHandler::new(250);
    let mut should_refresh = false;

    // Main loop
    while app.running {
        // Draw UI
        terminal.draw(|f| ui::draw(f, &app))?;

        // Handle refresh if requested
        if should_refresh && app.workspace.is_some() && app.client.is_some() {
            should_refresh = false;
            app.set_status("Refreshing...");
            terminal.draw(|f| ui::draw(f, &app))?;
            
            match app.current_view {
                View::Dashboard | View::Repositories => {
                    let _ = app.load_repositories().await;
                }
                View::PullRequests => {
                    let _ = app.load_pull_requests().await;
                }
                View::Issues => {
                    let _ = app.load_issues().await;
                }
                View::Pipelines => {
                    let _ = app.load_pipelines().await;
                }
            }
            
            app.set_status("Refreshed");
        }

        // Handle events
        match event_handler.next()? {
            Event::Key(key) => {
                // Check if refresh was requested
                if let crossterm::event::KeyCode::Char('r') = key.code {
                    should_refresh = true;
                }
                app.handle_key(key);
            }
            Event::Tick => {
                // Periodic tick for animations, etc.
            }
            Event::Resize(_, _) => {
                // Terminal will redraw automatically
            }
            _ => {}
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
