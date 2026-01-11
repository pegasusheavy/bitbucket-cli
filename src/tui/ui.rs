use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
};

use super::app::App;
use super::views::View;

/// Draw the application
pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    draw_header(f, app, chunks[0]);
    draw_main(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["Dashboard", "Repos", "PRs", "Issues", "Pipelines"];
    let selected = match app.current_view {
        View::Dashboard => 0,
        View::Repositories => 1,
        View::PullRequests => 2,
        View::Issues => 3,
        View::Pipelines => 4,
    };

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Bitbucket CLI "),
        )
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn draw_main(f: &mut Frame, app: &App, area: Rect) {
    match app.current_view {
        View::Dashboard => draw_dashboard(f, app, area),
        View::Repositories => draw_repositories(f, app, area),
        View::PullRequests => draw_pull_requests(f, app, area),
        View::Issues => draw_issues(f, app, area),
        View::Pipelines => draw_pipelines(f, app, area),
    }
}

fn draw_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Workspace info
    let workspace_text = match &app.workspace {
        Some(ws) => format!("Workspace: {}", ws),
        None => "No workspace selected".to_string(),
    };
    let workspace = Paragraph::new(workspace_text)
        .block(Block::default().borders(Borders::ALL).title(" Workspace "));
    f.render_widget(workspace, chunks[0]);

    // Dashboard menu
    let items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            Span::styled("📁 ", Style::default()),
            Span::raw("Repositories"),
            Span::styled(
                format!(" ({})", app.repositories.len()),
                Style::default().fg(Color::DarkGray),
            ),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("🔀 ", Style::default()),
            Span::raw("Pull Requests"),
            Span::styled(
                format!(" ({})", app.pull_requests.len()),
                Style::default().fg(Color::DarkGray),
            ),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("🐛 ", Style::default()),
            Span::raw("Issues"),
            Span::styled(
                format!(" ({})", app.issues.len()),
                Style::default().fg(Color::DarkGray),
            ),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("⚙️  ", Style::default()),
            Span::raw("Pipelines"),
            Span::styled(
                format!(" ({})", app.pipelines.len()),
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
    f.render_stateful_widget(list, chunks[1], &mut state);
}

fn draw_repositories(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = if app.repositories.is_empty() {
        vec![ListItem::new(
            "No repositories loaded. Press 'r' to refresh.",
        )]
    } else {
        app.repositories
            .iter()
            .map(|repo| {
                let private_badge = if repo.is_private.unwrap_or(false) {
                    "🔒"
                } else {
                    "🌐"
                };
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{} ", private_badge)),
                    Span::styled(&repo.full_name, Style::default().fg(Color::Cyan)),
                    Span::raw(" - "),
                    Span::styled(
                        repo.description.as_deref().unwrap_or("No description"),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]))
            })
            .collect()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Repositories "),
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

fn draw_pull_requests(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = if app.pull_requests.is_empty() {
        vec![ListItem::new(
            "No pull requests loaded. Press 'r' to refresh.",
        )]
    } else {
        app.pull_requests
            .iter()
            .map(|pr| {
                let state_color = match pr.state {
                    crate::models::PullRequestState::Open => Color::Green,
                    crate::models::PullRequestState::Merged => Color::Magenta,
                    crate::models::PullRequestState::Declined => Color::Red,
                    crate::models::PullRequestState::Superseded => Color::Yellow,
                };
                ListItem::new(Line::from(vec![
                    Span::styled(format!("[{}] ", pr.state), Style::default().fg(state_color)),
                    Span::styled(format!("#{} ", pr.id), Style::default().fg(Color::DarkGray)),
                    Span::raw(&pr.title),
                ]))
            })
            .collect()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Pull Requests "),
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

fn draw_issues(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = if app.issues.is_empty() {
        vec![ListItem::new("No issues loaded. Press 'r' to refresh.")]
    } else {
        app.issues
            .iter()
            .map(|issue| {
                let kind_icon = match issue.kind {
                    crate::models::IssueKind::Bug => "🐛",
                    crate::models::IssueKind::Enhancement => "✨",
                    crate::models::IssueKind::Proposal => "💡",
                    crate::models::IssueKind::Task => "📋",
                };
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{} ", kind_icon)),
                    Span::styled(
                        format!("#{} ", issue.id),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::raw(&issue.title),
                ]))
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Issues "))
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

fn draw_pipelines(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = if app.pipelines.is_empty() {
        vec![ListItem::new("No pipelines loaded. Press 'r' to refresh.")]
    } else {
        app.pipelines
            .iter()
            .map(|pipeline| {
                let (status_icon, status_color) = match pipeline.state.name {
                    crate::models::PipelineStateName::Pending => ("⏳", Color::Yellow),
                    crate::models::PipelineStateName::Building => ("🔄", Color::Blue),
                    crate::models::PipelineStateName::Completed => {
                        if let Some(result) = &pipeline.state.result {
                            match result.name {
                                crate::models::PipelineResultName::Successful => {
                                    ("✅", Color::Green)
                                }
                                crate::models::PipelineResultName::Failed => ("❌", Color::Red),
                                _ => ("⚪", Color::Gray),
                            }
                        } else {
                            ("⚪", Color::Gray)
                        }
                    }
                    crate::models::PipelineStateName::Halted => ("⛔", Color::Red),
                    crate::models::PipelineStateName::Paused => ("⏸️", Color::Yellow),
                };
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{} ", status_icon)),
                    Span::styled(
                        format!("#{} ", pipeline.build_number),
                        Style::default().fg(status_color),
                    ),
                    Span::raw(pipeline.target.ref_name.as_deref().unwrap_or("unknown")),
                ]))
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Pipelines "))
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

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let status_text = if let Some(error) = &app.error {
        Line::from(Span::styled(
            format!("Error: {}", error),
            Style::default().fg(Color::Red),
        ))
    } else if let Some(status) = &app.status {
        Line::from(Span::styled(status, Style::default().fg(Color::Yellow)))
    } else if app.loading {
        Line::from(Span::styled(
            "Loading...",
            Style::default().fg(Color::Yellow),
        ))
    } else {
        Line::from(vec![
            Span::styled("q", Style::default().fg(Color::Cyan)),
            Span::raw(" quit  "),
            Span::styled("1-5", Style::default().fg(Color::Cyan)),
            Span::raw(" switch view  "),
            Span::styled("j/k", Style::default().fg(Color::Cyan)),
            Span::raw(" navigate  "),
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" select  "),
            Span::styled("r", Style::default().fg(Color::Cyan)),
            Span::raw(" refresh"),
        ])
    };

    let footer =
        Paragraph::new(status_text).block(Block::default().borders(Borders::ALL).title(" Help "));
    f.render_widget(footer, area);
}
