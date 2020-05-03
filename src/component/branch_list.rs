use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    terminal::Frame,
    widgets::{Block, Borders, List, Text},
};

use crate::util::StatefulList;

pub struct BranchList;
impl BranchList {
    pub fn render<B: Backend>(f: &mut Frame<B>, chunk: &Rect, branches: &mut StatefulList<String>) {
        let items = List::new(branches.items.iter().map(Text::raw))
            .block(Block::default().borders(Borders::ALL).title("Branches"))
            .style(Style::default().fg(Color::Yellow))
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .modifier(Modifier::BOLD),
            )
            .highlight_symbol("➢ ");
        f.render_stateful_widget(items, *chunk, &mut branches.state);
    }
}
