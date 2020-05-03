use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    terminal::Frame,
    widgets::{Block, Borders, Paragraph, Text},
};

pub struct SearchInput;
impl SearchInput {
    pub fn render<B: Backend>(f: &mut Frame<B>, chunk: &Rect, input_str: &str) {
        let text = [Text::raw(input_str)];
        let input = Paragraph::new(text.iter())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input, *chunk);
    }
}
