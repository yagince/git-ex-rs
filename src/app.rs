use std::{
    collections::HashSet,
    io::{self, Write},
    path::Path,
};
use termion::{
    cursor::Goto, event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    component,
    util::{
        event::{Event, Events},
        StatefulList,
    },
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum InputMode {
    Command(Command),
    Search,
    Help,
    ShowLog,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Command {
    Checkout,
    #[allow(dead_code)]
    DeleteBranch,
}

impl Command {
    pub fn run(&self, app: &App) -> anyhow::Result<()> {
        match self {
            Command::Checkout => {
                Self::run_checkout(app)?;
            }
            Command::DeleteBranch => {
                Self::run_delete_branches(app)?;
            }
        }
        Ok(())
    }

    fn run_checkout(app: &App) -> anyhow::Result<()> {
        if let Some(ref branch) = app.selected_branch() {
            app.repo.checkout(branch)?;
        }
        Ok(())
    }

    fn run_delete_branches(app: &App) -> anyhow::Result<()> {
        app.selected
            .iter()
            .map(|x| app.repo.delete_branch(x))
            .collect()
    }
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub selected: HashSet<String>,
    pub repo: crate::git::Repository,
    pub branches: StatefulList<String>,
    pub all_branches: Vec<String>,
}

const TOP_MARGIN: u16 = 1;
const HELP_MESSAGE_HEIGHT: u16 = 1;
const TEXT_INPUT_HEIGHT: u16 = 3;
const LIST_WIDTH_PERCENTAGE: u16 = 40;
const LOG_LIMIT: usize = 40;

impl App {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<App> {
        let repo = crate::git::Repository::new(path)?;
        let branches = repo.branches()?;

        Ok(App {
            input: String::new(),
            input_mode: InputMode::Search,
            selected: HashSet::new(),
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

    pub fn checkout_mode(&mut self) {
        if self.selected_branch().is_some() {
            self.input_mode = InputMode::Command(Command::Checkout);
        }
    }

    pub fn search_mode(&mut self) {
        self.input_mode = InputMode::Search;
    }

    pub fn log_mode(&mut self) {
        self.input_mode = InputMode::ShowLog;
    }

    pub fn help_mode(&mut self) {
        self.input_mode = InputMode::Help;
    }

    pub fn delete_branch_mode(&mut self) {
        if !self.selected.is_empty() {
            self.input_mode = InputMode::Command(Command::DeleteBranch);
        }
    }

    pub fn selected_branch(&self) -> Option<&String> {
        self.branches.selected()
    }

    pub fn run_command(&self) -> anyhow::Result<()> {
        match self.input_mode {
            InputMode::Command(command) => command.run(self)?,
            _ => {}
        }
        Ok(())
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
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

                component::DefaultHelp::render(&mut f, &chunks[0], &self.input_mode);
                component::SearchInput::render(&mut f, &chunks[1], &self.input);

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
                    component::BranchList::render(
                        &mut f,
                        &chunks[0],
                        &mut self.branches,
                        self.repo.current_branch().unwrap(),
                    );
                    // selected
                    component::SelectedList::render(&mut f, &chunks[1], &self.selected);
                }

                {
                    match self.input_mode {
                        InputMode::Help => {
                            component::Help::render(&mut f);
                        }
                        InputMode::ShowLog => {
                            if let Some(ref branch_name) = self.selected_branch() {
                                let commits = self.repo.logs(branch_name, LOG_LIMIT).unwrap();
                                component::Logs::render(&mut f, commits);
                            }
                        }
                        InputMode::Command(command) => match command {
                            Command::Checkout => {
                                if let Some(ref branch_name) = self.selected_branch() {
                                    component::CheckoutConfirmation::render(&mut f, branch_name);
                                }
                            }
                            Command::DeleteBranch => {
                                component::DeleteBranchConfirmation::render(&mut f, &self.selected);
                            }
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
                    TOP_MARGIN + 2 + self.input.width() as u16,
                    TOP_MARGIN + HELP_MESSAGE_HEIGHT + TEXT_INPUT_HEIGHT - 1
                )
            )?;

            // stdout is buffered, flush it to see the effect immediately when hitting backspace
            io::stdout().flush().ok();

            // Handle input
            match events.next()? {
                Event::Input(input) => match self.input_mode {
                    InputMode::Command(_) => match input {
                        Key::Esc | Key::Ctrl('c') | Key::Char('n') | Key::Char('q') => {
                            self.search_mode();
                        }
                        Key::Char('y') | Key::Char('\n') => {
                            self.run_command()?;
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
                            if let Some(x) = self.branches.selected() {
                                match x.as_ref() {
                                    "master" => {}
                                    _ => {
                                        self.selected.insert(x.clone());
                                        self.branches.next();
                                    }
                                }
                            }
                        }
                        Key::Char(c) => {
                            self.input.push(c);
                            self.refresh_branches();
                        }
                        Key::Ctrl('h') | Key::Backspace | Key::Delete => {
                            self.input.pop();
                            self.refresh_branches();
                        }
                        Key::Ctrl('n') | Key::Down => {
                            self.branches.next();
                        }
                        Key::Ctrl('p') | Key::Up => {
                            self.branches.previous();
                        }
                        Key::Ctrl('o') => {
                            self.checkout_mode();
                        }
                        Key::Ctrl('l') => {
                            self.log_mode();
                        }
                        Key::Alt('h') => {
                            self.help_mode();
                        }
                        Key::Ctrl('d') => {
                            self.delete_branch_mode();
                        }
                        _ => {}
                    },
                    _ => match input {
                        Key::Char('y')
                        | Key::Char('q')
                        | Key::Char('\n')
                        | Key::Esc
                        | Key::Ctrl('n') => {
                            self.search_mode();
                        }
                        _ => {}
                    },
                },
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_branch_mode_when_empty_selected() -> anyhow::Result<()> {
        let mut app = App::new(std::env::current_dir()?)?;

        app.delete_branch_mode();

        Ok(assert_ne!(
            app.input_mode,
            InputMode::Command(Command::DeleteBranch)
        ))
    }

    #[test]
    fn test_delete_branch_mode_when_not_empty_selected() -> anyhow::Result<()> {
        let mut app = App::new(std::env::current_dir()?)?;
        app.selected.insert("Hoge".into());

        app.delete_branch_mode();

        Ok(assert_eq!(
            app.input_mode,
            InputMode::Command(Command::DeleteBranch)
        ))
    }
}
