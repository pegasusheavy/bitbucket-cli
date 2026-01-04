/// Issue browser view
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::models::{Issue, IssueKind, IssuePriority, IssueState};
use crate::tui::app::App;

/// Issue list view
pub struct IssuesView;

impl IssuesView {
    /// Render the issue browser
    pub fn render(f: &mut Frame, app: &App, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60), // List
                Constraint::Percentage(40), // Details
            ])
            .split(area);

        Self::render_list(f, app, chunks[0]);
        Self::render_details(f, app, chunks[1]);
    }

    fn render_list(f: &mut Frame, app: &App, area: Rect) {
        let items: Vec<ListItem> = if app.issues.is_empty() {
            vec![
                ListItem::new(Line::from(Span::styled(
                    "No issues loaded",
                    Style::default().fg(Color::DarkGray),
                ))),
                ListItem::new(Line::from("")),
                ListItem::new(Line::from(Span::styled(
                    "Press 'r' to refresh",
                    Style::default().fg(Color::Yellow),
                ))),
            ]
        } else {
            app.issues
                .iter()
                .map(|issue| Self::issue_to_list_item(issue))
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
        if !app.issues.is_empty() {
            state.select(Some(app.view_state.selected_index));
        }
        f.render_stateful_widget(list, area, &mut state);
    }

    fn render_details(f: &mut Frame, app: &App, area: Rect) {
        let content = if let Some(issue) = app.issues.get(app.view_state.selected_index) {
            let state_color = Self::state_color(&issue.state);
            let priority_color = Self::priority_color(&issue.priority);

            vec![
                Line::from(vec![
                    Span::styled(
                        format!("#{} ", issue.id),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(&issue.title, Style::default().add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{}", issue.state), Style::default().fg(state_color)),
                ]),
                Line::from(vec![
                    Span::styled("Type: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(format!("{}", issue.kind)),
                ]),
                Line::from(vec![
                    Span::styled("Priority: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{}", issue.priority),
                        Style::default().fg(priority_color),
                    ),
                ]),
                Line::from(""),
                if let Some(reporter) = &issue.reporter {
                    Line::from(vec![
                        Span::styled("Reporter: ", Style::default().fg(Color::DarkGray)),
                        Span::raw(&reporter.display_name),
                    ])
                } else {
                    Line::from("")
                },
                if let Some(assignee) = &issue.assignee {
                    Line::from(vec![
                        Span::styled("Assignee: ", Style::default().fg(Color::DarkGray)),
                        Span::raw(&assignee.display_name),
                    ])
                } else {
                    Line::from(vec![
                        Span::styled("Assignee: ", Style::default().fg(Color::DarkGray)),
                        Span::styled("Unassigned", Style::default().fg(Color::DarkGray)),
                    ])
                },
                Line::from(""),
                Line::from(vec![
                    Span::styled("Created: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(issue.created_on.format("%Y-%m-%d %H:%M").to_string()),
                ]),
                Line::from(""),
                if issue
                    .content
                    .as_ref()
                    .and_then(|c| c.raw.as_ref())
                    .is_some()
                {
                    Line::from(vec![Span::styled(
                        "Description: ",
                        Style::default().fg(Color::DarkGray),
                    )])
                } else {
                    Line::from("")
                },
            ]
        } else {
            vec![Line::from(Span::styled(
                "Select an issue to view details",
                Style::default().fg(Color::DarkGray),
            ))]
        };

        let details = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title(" Details "));
        f.render_widget(details, area);
    }

    fn issue_to_list_item(issue: &Issue) -> ListItem<'static> {
        let kind_icon = match issue.kind {
            IssueKind::Bug => "🐛",
            IssueKind::Enhancement => "✨",
            IssueKind::Proposal => "💡",
            IssueKind::Task => "📋",
        };

        let state_color = Self::state_color(&issue.state);

        ListItem::new(Line::from(vec![
            Span::raw(format!("{} ", kind_icon)),
            Span::styled(format!("#{} ", issue.id), Style::default().fg(state_color)),
            Span::raw(issue.title.chars().take(45).collect::<String>()),
        ]))
    }

    fn state_color(state: &IssueState) -> Color {
        match state {
            IssueState::New => Color::Cyan,
            IssueState::Open => Color::Green,
            IssueState::Resolved => Color::Blue,
            IssueState::OnHold => Color::Yellow,
            IssueState::Invalid | IssueState::Duplicate | IssueState::Wontfix => Color::DarkGray,
            IssueState::Closed => Color::Magenta,
        }
    }

    fn priority_color(priority: &IssuePriority) -> Color {
        match priority {
            IssuePriority::Trivial => Color::DarkGray,
            IssuePriority::Minor => Color::White,
            IssuePriority::Major => Color::Yellow,
            IssuePriority::Critical => Color::Red,
            IssuePriority::Blocker => Color::LightRed,
        }
    }
}
