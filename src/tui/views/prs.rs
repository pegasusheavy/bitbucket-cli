/// Pull request browser view
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::models::{PullRequest, PullRequestState};
use crate::tui::app::App;

/// Pull request list view
pub struct PrsView;

impl PrsView {
    /// Render the pull request browser
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
        let items: Vec<ListItem> = if app.pull_requests.is_empty() {
            vec![
                ListItem::new(Line::from(Span::styled(
                    "No pull requests loaded",
                    Style::default().fg(Color::DarkGray),
                ))),
                ListItem::new(Line::from("")),
                ListItem::new(Line::from(Span::styled(
                    "Press 'r' to refresh",
                    Style::default().fg(Color::Yellow),
                ))),
            ]
        } else {
            app.pull_requests
                .iter()
                .map(|pr| Self::pr_to_list_item(pr))
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
        if !app.pull_requests.is_empty() {
            state.select(Some(app.view_state.selected_index));
        }
        f.render_stateful_widget(list, area, &mut state);
    }

    fn render_details(f: &mut Frame, app: &App, area: Rect) {
        let content = if let Some(pr) = app.pull_requests.get(app.view_state.selected_index) {
            let state_color = Self::state_color(&pr.state);

            vec![
                Line::from(vec![
                    Span::styled(format!("#{} ", pr.id), Style::default().fg(Color::DarkGray)),
                    Span::styled(&pr.title, Style::default().add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{}", pr.state), Style::default().fg(state_color)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Author: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(&pr.author.display_name),
                ]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    "Branches: ",
                    Style::default().fg(Color::DarkGray),
                )]),
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(&pr.source.branch.name, Style::default().fg(Color::Cyan)),
                    Span::styled(" → ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        &pr.destination.branch.name,
                        Style::default().fg(Color::Green),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Created: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(pr.created_on.format("%Y-%m-%d %H:%M").to_string()),
                ]),
                Line::from(vec![
                    Span::styled("Updated: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(pr.updated_on.format("%Y-%m-%d %H:%M").to_string()),
                ]),
                Line::from(""),
                if let Some(count) = pr.comment_count {
                    Line::from(vec![
                        Span::styled("Comments: ", Style::default().fg(Color::DarkGray)),
                        Span::raw(format!("{}", count)),
                    ])
                } else {
                    Line::from("")
                },
            ]
        } else {
            vec![Line::from(Span::styled(
                "Select a pull request to view details",
                Style::default().fg(Color::DarkGray),
            ))]
        };

        let details = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title(" Details "));
        f.render_widget(details, area);
    }

    fn pr_to_list_item(pr: &PullRequest) -> ListItem<'static> {
        let state_color = Self::state_color(&pr.state);
        let state_icon = match pr.state {
            PullRequestState::Open => "○",
            PullRequestState::Merged => "●",
            PullRequestState::Declined => "✗",
            PullRequestState::Superseded => "◌",
        };

        ListItem::new(Line::from(vec![
            Span::styled(format!("{} ", state_icon), Style::default().fg(state_color)),
            Span::styled(format!("#{} ", pr.id), Style::default().fg(Color::DarkGray)),
            Span::raw(pr.title.chars().take(50).collect::<String>()),
        ]))
    }

    fn state_color(state: &PullRequestState) -> Color {
        match state {
            PullRequestState::Open => Color::Green,
            PullRequestState::Merged => Color::Magenta,
            PullRequestState::Declined => Color::Red,
            PullRequestState::Superseded => Color::Yellow,
        }
    }
}
