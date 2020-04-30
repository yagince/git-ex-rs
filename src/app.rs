use std::collections::HashSet;

use git2::Repository;

use crate::util::StatefulList;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum InputMode {
    Command(Command),
    Search,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Command {
    Checkout,
    Log,
    #[allow(dead_code)]
    DeleteBranch,
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub selected: HashSet<String>,
    pub repo: git2::Repository,
    pub branches: StatefulList<String>,
    pub all_branches: Vec<String>,
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
        self.input_mode = InputMode::Command(Command::Log);
    }

    pub fn selected_branch(&self) -> Option<&String> {
        self.branches.selected()
    }

    pub fn run_command(&self) -> anyhow::Result<()> {
        match self.input_mode {
            InputMode::Command(command) => match command {
                Command::Checkout => {
                    self.run_checkout()?;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn run_checkout(&self) -> anyhow::Result<()> {
        if let Some(ref branch) = self.selected_branch() {
            self.repo
                .find_branch(branch, git2::BranchType::Local)
                .and_then(|branch| self.repo.set_head(branch.get().name().unwrap()))?;
        }
        Ok(())
    }
}
