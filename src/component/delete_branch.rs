use std::collections::HashSet;
use tui::{
    backend::Backend,
    terminal::Frame,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Text},
};

use crate::util;

pub struct DeleteBranchConfirmation;
impl DeleteBranchConfirmation {
    pub fn render<B: Backend> (f: &mut Frame<B>, selected: &HashSet<String>) {
        let mut text = vec![
            Text::raw("Would you like to "),
            Text::styled("delete branches", Style::default().fg(Color::Green)),
            Text::raw(" ?"),
            Text::raw("\n"),
            Text::raw("Enter: "),
            Text::styled("y", Style::default().fg(Color::Green)),
            Text::raw(" or "),
            Text::styled("n", Style::default().fg(Color::LightMagenta)),
            Text::raw("\n\n"),
            Text::styled("selected branches:\n", Style::default().fg(Color::Yellow)),
        ];
        selected.iter().for_each(|branch_name| {
            text.push(Text::raw("--> "));
            text.push(Text::styled(branch_name, Style::default().fg(Color::Cyan)));
            text.push(Text::raw("\n"));
        });

        let paragraph = Paragraph::new(text.iter())
            .block(
                Block::default()
                    .title("Delete Branch")
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Left)
            .wrap(true);

        let area = util::centered_rect(60, 60, f.size());

        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(paragraph, area);
    }
}
