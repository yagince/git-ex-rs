use std::{
    collections::HashSet,
    env,
    io::{self, Write},
};
use termion::{
    cursor::Goto, event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, Paragraph, Text},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

use git2::Repository;

use gitex::util::{
    self,
    event::{Event, Events},
    StatefulList,
};

#[derive(Debug, Clone, PartialEq)]
enum InputMode {
    Command,
    Search,
}

struct App {
    input: String,
    input_mode: InputMode,
    selected: HashSet<String>,
    #[allow(dead_code)]
    repo: git2::Repository,
    branches: StatefulList<String>,
    all_branches: Vec<String>,
    checkout: Option<String>,
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
            input: String::new(),
            input_mode: InputMode::Search,
            selected: HashSet::new(),
            repo: repo,
            all_branches: branches.clone(),
            branches: StatefulList::with_items(branches),
            checkout: None,
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
                InputMode::Command => "Press q or Ctrl+c to exit, e to start search mode.",
                InputMode::Search => "Press Esc or Ctrl+c to exit, Enter to record the message",
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
                // selected
                let selected = app
                    .selected
                    .iter()
                    .enumerate()
                    .map(|(i, m)| Text::raw(format!("{}: {}", i, m)));
                let selected = List::new(selected)
                    .block(Block::default().borders(Borders::ALL).title("Selected"));
                f.render_widget(selected, chunks[0]);

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

            {
                if let Some(ref checkout) = app.checkout {
                    let text = [
                        Text::raw("You checkout "),
                        Text::styled(checkout, Style::default().fg(Color::Green)),
                        Text::raw(" ?"),
                    ];
                    let paragraph = Paragraph::new(text.iter())
                        .block(Block::default().title("Popup").borders(Borders::ALL))
                        .alignment(Alignment::Left)
                        .wrap(true);

                    let area = util::centered_rect(60, 10, f.size());

                    f.render_widget(Clear, area); //this clears out the background
                    f.render_widget(paragraph, area);
                }
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
                InputMode::Command => match input {
                    Key::Esc | Key::Char('q') | Key::Ctrl('c') => {
                        app.input_mode = InputMode::Search;
                        app.checkout = None;
                    }
                    _ => {}
                },
                InputMode::Search => match input {
                    // exit
                    Key::Esc | Key::Ctrl('c') => match app.checkout {
                        Some(_) => app.checkout = None,
                        _ => break,
                    },
                    // press Enter
                    Key::Char('\n') => {
                        if let Some(x) = app.branches.selected() {
                            app.selected.insert(x.clone());
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
                    Key::Ctrl('o') => {
                        if let Some(x) = app.selected.iter().next() {
                            app.checkout = Some(x.to_owned());
                            app.input_mode = InputMode::Command;
                        }
                    }
                    _ => {}
                },
            },
            _ => {}
        }
    }
    Ok(())
}
