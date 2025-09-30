use ratatui::{
    layout::{Alignment},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw_fullscreen_page(f: &mut ratatui::Frame, title: &str) {
    let size = f.size();

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(vec![
        Line::from(Span::styled(
            format!("{} Screen", title),
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Press ESC or q to return to the main menu.")),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Full-screen layout for this screen.")),
    ])
    .block(block)
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });

    f.render_widget(paragraph, size);
}
