/// Dashboard view - main overview of workspace
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::tui::app::App;

/// Dashboard widget showing workspace overview
pub struct DashboardView;

impl DashboardView {
    /// Render the dashboard
    pub fn render(f: &mut Frame, app: &App, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5), // Header with workspace info
                Constraint::Length(8), // Quick stats
                Constraint::Min(0),    // Quick access menu
            ])
            .split(area);

        Self::render_header(f, app, chunks[0]);
        Self::render_stats(f, app, chunks[1]);
        Self::render_menu(f, app, chunks[2]);
    }

    fn render_header(f: &mut Frame, app: &App, area: Rect) {
        let workspace_info = match &app.workspace {
            Some(ws) => vec![
                Line::from(vec![
                    Span::styled("Workspace: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        ws,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "Welcome to Bitbucket CLI TUI",
                    Style::default().fg(Color::White),
                )),
            ],
            None => vec![
                Line::from(Span::styled(
                    "No workspace selected",
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "Use --workspace flag or set a default workspace",
                    Style::default().fg(Color::DarkGray),
                )),
            ],
        };

        let header = Paragraph::new(workspace_info).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 🌐 Bitbucket "),
        );
        f.render_widget(header, area);
    }

    fn render_stats(f: &mut Frame, app: &App, area: Rect) {
        let stats_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        // Repositories stat
        let repos_stat = Paragraph::new(vec![
            Line::from(Span::styled(
                format!("{}", app.repositories.len()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "Repositories",
                Style::default().fg(Color::DarkGray),
            )),
        ])
        .block(Block::default().borders(Borders::ALL).title(" 📁 "));
        f.render_widget(repos_stat, stats_layout[0]);

        // Pull requests stat
        let open_prs = app
            .pull_requests
            .iter()
            .filter(|pr| pr.state == crate::models::PullRequestState::Open)
            .count();
        let prs_stat = Paragraph::new(vec![
            Line::from(Span::styled(
                format!("{}", open_prs),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "Open PRs",
                Style::default().fg(Color::DarkGray),
            )),
        ])
        .block(Block::default().borders(Borders::ALL).title(" 🔀 "));
        f.render_widget(prs_stat, stats_layout[1]);

        // Issues stat
        let open_issues = app
            .issues
            .iter()
            .filter(|i| {
                i.state == crate::models::IssueState::Open
                    || i.state == crate::models::IssueState::New
            })
            .count();
        let issues_stat = Paragraph::new(vec![
            Line::from(Span::styled(
                format!("{}", open_issues),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "Open Issues",
                Style::default().fg(Color::DarkGray),
            )),
        ])
        .block(Block::default().borders(Borders::ALL).title(" 🐛 "));
        f.render_widget(issues_stat, stats_layout[2]);

        // Pipelines stat
        let running_pipelines = app
            .pipelines
            .iter()
            .filter(|p| p.state.name == crate::models::PipelineStateName::Building)
            .count();
        let pipelines_stat = Paragraph::new(vec![
            Line::from(Span::styled(
                format!("{}", running_pipelines),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "Running",
                Style::default().fg(Color::DarkGray),
            )),
        ])
        .block(Block::default().borders(Borders::ALL).title(" ⚙️ "));
        f.render_widget(pipelines_stat, stats_layout[3]);
    }

    fn render_menu(f: &mut Frame, app: &App, area: Rect) {
        let items: Vec<ListItem> = vec![
            ListItem::new(Line::from(vec![
                Span::styled("📁 ", Style::default()),
                Span::styled(
                    "Repositories",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " - Browse and manage repositories",
                    Style::default().fg(Color::DarkGray),
                ),
            ])),
            ListItem::new(Line::from(vec![
                Span::styled("🔀 ", Style::default()),
                Span::styled(
                    "Pull Requests",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " - Review and merge code",
                    Style::default().fg(Color::DarkGray),
                ),
            ])),
            ListItem::new(Line::from(vec![
                Span::styled("🐛 ", Style::default()),
                Span::styled("Issues", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    " - Track bugs and tasks",
                    Style::default().fg(Color::DarkGray),
                ),
            ])),
            ListItem::new(Line::from(vec![
                Span::styled("⚙️  ", Style::default()),
                Span::styled("Pipelines", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    " - Monitor CI/CD builds",
                    Style::default().fg(Color::DarkGray),
                ),
            ])),
        ];

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Quick Access "),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        let mut state = ratatui::widgets::ListState::default();
        state.select(Some(app.view_state.selected_index));
        f.render_stateful_widget(list, area, &mut state);
    }
}
