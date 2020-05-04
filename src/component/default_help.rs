use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    terminal::Frame,
    widgets::{Paragraph, Text},
};

use crate::app::InputMode;

const HELP_COMMAND: &str = "Press q or Ctrl+c to exit, e to start search mode.";
const HELP_SEARCH: &str = "Press Esc or Ctrl+c to exit, Enter to record the message. (Help: Alt+h)";
const HELP_OTHER: &str = "Press Esc or q or Ctrl+c or Enter back to Search";

pub struct DefaultHelp;
impl DefaultHelp {
    pub fn render<B: Backend>(f: &mut Frame<B>, chunk: &Rect, mode: &InputMode) {
        let msg = match mode {
            InputMode::Command(_) => HELP_COMMAND,
            InputMode::Search => HELP_SEARCH,
            _ => HELP_OTHER,
        };

        // help message
        let text = [Text::raw(msg)];
        let help_message = Paragraph::new(text.iter()).style(Style::default().fg(Color::Cyan));
        f.render_widget(help_message, *chunk);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui::{
        backend::TestBackend,
        buffer::Buffer,
        layout::{Constraint, Direction, Layout},
        Terminal,
    };

    use crate::app::Command;

    macro_rules! assert_render {
        ($mode:expr, $message:expr) => {
            let width: u16 = 200;
            let render = |mode: &InputMode| {
                let backend = TestBackend::new(width, 1);
                let mut terminal = Terminal::new(backend).unwrap();
                terminal
                    .draw(|mut f| {
                        let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Length(2), Constraint::Min(1)].as_ref())
                            .split(f.size());
                        DefaultHelp::render(&mut f, &chunks[0], mode);
                    })
                    .unwrap();

                terminal.backend().buffer().clone()
            };

            {
                let message = $message;
                let mut expect =
                    Buffer::with_lines(vec![format!("{:1$}", message.clone(), width as usize)]);
                (0..message.len() as u16).for_each(|i| {
                    expect.get_mut(i, 0).set_fg(Color::Cyan);
                });

                assert_eq!(render(&$mode), expect);
            }
        };
    }

    #[test]
    fn test_render_search() {
        assert_render!(InputMode::Search, HELP_SEARCH);
    }
    #[test]
    fn test_render_command() {
        assert_render!(InputMode::Command(Command::Checkout), HELP_COMMAND);
        assert_render!(InputMode::Command(Command::DeleteBranch), HELP_COMMAND);
    }
    #[test]
    fn test_render_other() {
        assert_render!(InputMode::Help, HELP_OTHER);
        assert_render!(InputMode::ShowLog, HELP_OTHER);
    }
}
