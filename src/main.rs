use std::{
    env,
    io::{self, Write},
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

use git_ex::{
    app::App,
    component,
    util::event::{Event, Events},
    Command, InputMode,
};

const TOP_MARGIN: u16 = 1;
const HELP_MESSAGE_HEIGHT: u16 = 1;
const TEXT_INPUT_HEIGHT: u16 = 3;
const LIST_WIDTH_PERCENTAGE: u16 = 40;
const LOG_LIMIT: usize = 40;

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
                InputMode::Command(_) => "Press q or Ctrl+c to exit, e to start search mode.",
                InputMode::Search => {
                    "Press Esc or Ctrl+c to exit, Enter to record the message. (Help: Alt+h)"
                }
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
                f.render_stateful_widget(items, chunks[0], &mut app.branches.state);

                // selected
                let selected = app
                    .selected
                    .iter()
                    .enumerate()
                    .map(|(i, m)| Text::raw(format!("{}: {}", i, m)));
                let selected = List::new(selected)
                    .block(Block::default().borders(Borders::ALL).title("Selected"));
                f.render_widget(selected, chunks[1]);
            }

            {
                match app.input_mode {
                    InputMode::Command(command) => match command {
                        Command::Help => {
                            component::Help::render(&mut f);
                        }
                        Command::Checkout => {
                            if let Some(ref branch_name) = app.selected_branch() {
                                component::CheckoutConfirmation::render(&mut f, branch_name);
                            }
                        }
                        Command::Log => {
                            if let Some(ref branch_name) = app.selected_branch() {
                                let commits = app._repo.logs(branch_name, LOG_LIMIT).unwrap();
                                component::Logs::render(&mut f, commits);
                            }
                        }
                        _ => {}
                    },
                    _ => {}
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
                InputMode::Command(_) => match input {
                    Key::Esc | Key::Ctrl('c') | Key::Char('n') | Key::Char('q') => {
                        app.search_mode();
                    }
                    Key::Char('y') => {
                        app.run_command()?;
                        break;
                    }
                    _ => {}
                },
                InputMode::Search => match input {
                    // exit
                    Key::Esc | Key::Ctrl('c') => {
                        break;
                    }
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
                    Key::Ctrl('h') | Key::Backspace | Key::Delete => {
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
                        app.checkout_mode();
                    }
                    Key::Ctrl('l') => {
                        app.log_mode();
                    }
                    Key::Alt('h') => {
                        app.help_mode();
                    }
                    _ => {}
                },
            },
            _ => {}
        }
    }
    Ok(())
}
