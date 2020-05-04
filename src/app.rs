use std::{collections::HashSet, path::Path};

use crate::util::StatefulList;

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
            _ => {}
        }
        Ok(())
    }

    fn run_checkout(app: &App) -> anyhow::Result<()> {
        if let Some(ref branch) = app.selected_branch() {
            app.repo.checkout(branch)?;
        }
        Ok(())
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
