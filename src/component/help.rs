use crate::util;

use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    terminal::Frame,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Text},
};

pub struct Help;
impl Help {
    pub fn render<B: Backend>(f: &mut Frame<B>) {
        let text = [
            // Help
            Text::styled("Show Help      ", Style::default().fg(Color::Green)),
            Text::raw(": Alt+h"),
            Text::raw("\n"),
            // Checkout
            Text::styled("Checkout Branch", Style::default().fg(Color::Green)),
            Text::raw(": Ctrl+o"),
            Text::raw("\n"),
            // Delete Branches
            Text::styled("Delete Branches", Style::default().fg(Color::Green)),
            Text::raw(": Ctrl+d"),
            Text::raw("\n"),
            // Log
            Text::styled("Show log       ", Style::default().fg(Color::Green)),
            Text::raw(": Ctrl+l"),
            Text::raw("\n"),
        ];
        let paragraph = Paragraph::new(text.iter())
            .block(
                Block::default()
                    .title("  Help  ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Left)
            .wrap(true);

        let area = util::centered_rect(50, 60, f.size());

        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(paragraph, area);
    }
}
