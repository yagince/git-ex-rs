use tui::{
    backend::Backend,
    layout::Rect,
    terminal::Frame,
    widgets::{Paragraph, Text},
};

use crate::app::InputMode;

pub struct DefaultHelp;
impl DefaultHelp {
    pub fn render<B: Backend>(f: &mut Frame<B>, chunk: &Rect, mode: &InputMode) {
        let msg = match mode {
            InputMode::Command(_) => "Press q or Ctrl+c to exit, e to start search mode.",
            InputMode::Search => {
                "Press Esc or Ctrl+c to exit, Enter to record the message. (Help: Alt+h)"
            }
        };

        // help message
        let text = [Text::raw(msg)];
        let help_message = Paragraph::new(text.iter());
        f.render_widget(help_message, *chunk);
    }
}
