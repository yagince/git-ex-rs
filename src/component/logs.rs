use crate::util;

use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    terminal::Frame,
    widgets::{Block, Borders, Clear, Paragraph, Text},
};

pub struct Logs;
impl Logs {
    pub fn render<B: Backend>(f: &mut Frame<B>, commits: Vec<crate::git::Commit>) {
        let text = commits
            .into_iter()
            .flat_map(|log| {
                vec![
                    Text::styled(log.id, Style::default().fg(Color::Yellow)),
                    Text::raw(" "),
                    Text::raw(log.message),
                ]
            })
            .collect::<Vec<_>>();

        let paragraph = Paragraph::new(text.iter())
            .block(Block::default().title("Log").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .wrap(true);

        let area = util::centered_rect(60, 50, f.size());

        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(paragraph, area);
    }
}
