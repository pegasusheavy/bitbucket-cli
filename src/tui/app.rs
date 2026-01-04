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
                self.set_status("Refreshing...");
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
            }
        }
        Err(e) => {
            app.set_error(&format!("Not authenticated: {}", e));
        }
    }

    // Create event handler
    let event_handler = EventHandler::new(250);

    // Main loop
    while app.running {
        // Draw UI
        terminal.draw(|f| ui::draw(f, &app))?;

        // Handle events
        match event_handler.next()? {
            Event::Key(key) => {
                app.handle_key(key);
            }
            Event::Tick => {
                // Could refresh data here
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
