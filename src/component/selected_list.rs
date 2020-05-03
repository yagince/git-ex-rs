use tui::{
    backend::Backend,
    layout::Rect,
    terminal::Frame,
    widgets::{Block, Borders, List, Text},
};

use std::collections::HashSet;

pub struct SelectedList;
impl SelectedList {
    pub fn render<B: Backend>(f: &mut Frame<B>, chunk: &Rect, selected: &HashSet<String>) {
        let selected = selected
            .iter()
            .enumerate()
            .map(|(i, m)| Text::raw(format!("{}: {}", i, m)));
        let selected =
            List::new(selected).block(Block::default().borders(Borders::ALL).title("Selected"));
        f.render_widget(selected, *chunk);
    }
}
