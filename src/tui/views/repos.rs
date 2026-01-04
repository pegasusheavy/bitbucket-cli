/// Repository browser view
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::models::Repository;
use crate::tui::app::App;

/// Repository list view
pub struct ReposView;

impl ReposView {
    /// Render the repository browser
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
        let items: Vec<ListItem> = if app.repositories.is_empty() {
            vec![
                ListItem::new(Line::from(Span::styled(
                    "No repositories loaded",
                    Style::default().fg(Color::DarkGray),
                ))),
                ListItem::new(Line::from("")),
                ListItem::new(Line::from(Span::styled(
                    "Press 'r' to refresh",
                    Style::default().fg(Color::Yellow),
                ))),
            ]
        } else {
            app.repositories
                .iter()
                .map(|repo| Self::repo_to_list_item(repo))
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
        if !app.repositories.is_empty() {
            state.select(Some(app.view_state.selected_index));
        }
        f.render_stateful_widget(list, area, &mut state);
    }

    fn render_details(f: &mut Frame, app: &App, area: Rect) {
        let content = if let Some(repo) = app.repositories.get(app.view_state.selected_index) {
            vec![
                Line::from(vec![Span::styled(
                    &repo.full_name,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Description: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(repo.description.as_deref().unwrap_or("No description")),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Private: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(if repo.is_private { "Yes" } else { "No" }),
                ]),
                Line::from(vec![
                    Span::styled("SCM: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(&repo.scm),
                ]),
                Line::from(vec![
                    Span::styled("Language: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(repo.language.as_deref().unwrap_or("Not specified")),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Main branch: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(
                        repo.mainbranch
                            .as_ref()
                            .map(|b| b.name.as_str())
                            .unwrap_or("main"),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Created: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(
                        repo.created_on
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or_default(),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Updated: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(
                        repo.updated_on
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or_default(),
                    ),
                ]),
            ]
        } else {
            vec![Line::from(Span::styled(
                "Select a repository to view details",
                Style::default().fg(Color::DarkGray),
            ))]
        };

        let details = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title(" Details "));
        f.render_widget(details, area);
    }

    fn repo_to_list_item(repo: &Repository) -> ListItem<'static> {
        let private_badge = if repo.is_private { "🔒" } else { "🌐" };
        let lang_badge = repo.language.as_deref().unwrap_or("");

        ListItem::new(Line::from(vec![
            Span::raw(format!("{} ", private_badge)),
            Span::styled(repo.full_name.clone(), Style::default().fg(Color::Cyan)),
            if !lang_badge.is_empty() {
                Span::styled(
                    format!(" [{}]", lang_badge),
                    Style::default().fg(Color::Yellow),
                )
            } else {
                Span::raw("")
            },
        ]))
    }
}
