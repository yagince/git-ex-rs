use tui::{
    backend::Backend,
    terminal::Frame,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Text},
};

use crate::util;

pub struct CheckoutConfirmation;
impl CheckoutConfirmation {
    pub fn render<B: Backend> (f: &mut Frame<B>, branch_name: &str) {
        let text = [
            Text::raw("Would you like to checkout "),
            Text::styled(branch_name.to_owned(), Style::default().fg(Color::Green)),
            Text::raw(" ?"),
        ];
        let paragraph = Paragraph::new(text.iter())
            .block(
                Block::default()
                    .title("Checkout Branch")
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Left)
            .wrap(true);

        let area = util::centered_rect(60, 10, f.size());

        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(paragraph, area);
    }
}
