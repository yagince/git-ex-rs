use std::{
    env,
    io::{self, Write},
    collections::HashSet,
};
use termion::{
    cursor::Goto, event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, Paragraph, Text},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

use git2::Repository;

use gitex::util::{
    event::{Event, Events},
    StatefulList,
};

#[derive(Debug, Clone, PartialEq)]
enum InputMode {
    #[allow(dead_code)]
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: HashSet<String>,
    #[allow(dead_code)]
    repo: git2::Repository,
    branches: StatefulList<String>,
    all_branches: Vec<String>,
}

impl App {
    pub fn new(repo: Repository) -> anyhow::Result<App> {
        let branches = repo
            .branches(Some(git2::BranchType::Local))?
            .map(|x| {
                let (branch, _) = x?;
                Ok(branch.name()?.unwrap().to_owned())
            })
            .collect::<anyhow::Result<Vec<String>>>()?;
        Ok(App {
            input: "".to_owned(),
            input_mode: InputMode::Editing,
            messages: HashSet::new(),
            repo: repo,
            all_branches: branches.clone(),
            branches: StatefulList::with_items(branches),
        })
    }

    pub fn refresh_branches(&mut self) {
        self.branches.set_items(
            self.all_branches
                .iter()
                .filter(|x| x.contains(&self.input))
                .cloned()
                .collect(),
        );
    }
}

const TOP_MARGIN: u16 = 1;
const HELP_MESSAGE_HEIGHT: u16 = 1;
const TEXT_INPUT_HEIGHT: u16 = 3;
const LIST_WIDTH_PERCENTAGE: u16 = 40;

fn main() -> anyhow::Result<()> {
    let repo = Repository::open(env::current_dir()?).expect("repo not found");

    // Create default app state
    let mut app = App::new(repo)?;

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let events = Events::new();

    loop {
        // Draw UI
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(TOP_MARGIN)
                .constraints(
                    [
                        Constraint::Length(HELP_MESSAGE_HEIGHT),
                        Constraint::Length(TEXT_INPUT_HEIGHT),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let msg = match app.input_mode {
                InputMode::Normal => "Press q to exit, e to start editing.",
                InputMode::Editing => "Press Esc or Ctrl+c to exit, Enter to record the message",
            };

            // help message
            let text = [Text::raw(msg)];
            let help_message = Paragraph::new(text.iter());
            f.render_widget(help_message, chunks[0]);

            // input
            let text = [Text::raw(&app.input)];
            let input = Paragraph::new(text.iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input, chunks[1]);

            {
                // main area
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(LIST_WIDTH_PERCENTAGE),
                            Constraint::Percentage(100 - LIST_WIDTH_PERCENTAGE),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[2]);
                // messages
                let messages = app
                    .messages
                    .iter()
                    .enumerate()
                    .map(|(i, m)| Text::raw(format!("{}: {}", i, m)));
                let messages = List::new(messages)
                    .block(Block::default().borders(Borders::ALL).title("Messages"));
                f.render_widget(messages, chunks[0]);

                // branches
                let items = List::new(app.branches.items.iter().map(Text::raw))
                    .block(Block::default().borders(Borders::ALL).title("Branches"))
                    .style(Style::default().fg(Color::Yellow))
                    .highlight_style(
                        Style::default()
                            .fg(Color::LightGreen)
                            .modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("➢ ");
                f.render_stateful_widget(items, chunks[1], &mut app.branches.state);
            }
        })?;

        // Put the cursor back inside the input box
        write!(
            terminal.backend_mut(),
            "{}",
            Goto(
                TOP_MARGIN + 2 + app.input.width() as u16,
                TOP_MARGIN + HELP_MESSAGE_HEIGHT + TEXT_INPUT_HEIGHT - 1
            )
        )?;

        // stdout is buffered, flush it to see the effect immediately when hitting backspace
        io::stdout().flush().ok();

        // Handle input
        match events.next()? {
            Event::Input(input) => match app.input_mode {
                InputMode::Normal => {}
                InputMode::Editing => match input {
                    // exit
                    Key::Esc | Key::Ctrl('c') => {
                        break;
                    }
                    // press Enter
                    Key::Char('\n') => {
                        if let Some(x) = app.branches.selected() {
                            app.messages.insert(x.clone());
                        }
                    }
                    Key::Char(c) => {
                        app.input.push(c);
                        app.refresh_branches();
                    }
                    Key::Backspace => {
                        app.input.pop();
                        app.refresh_branches();
                    }
                    Key::Ctrl('n') | Key::Down => {
                        app.branches.next();
                    }
                    Key::Ctrl('p') | Key::Up => {
                        app.branches.previous();
                    }
                    _ => {}
                },
            },
            _ => {}
        }
    }
    Ok(())
}
