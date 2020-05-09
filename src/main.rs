use git_ex::{app::App, cmd::StartBranchOpts};
use std::env;

use anyhow::anyhow;
use clap::Clap;
#[derive(Debug, Clone, PartialEq, Clap)]
#[clap(author, about, version, name = "git-ex")]
struct Opts {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Debug, Clone, PartialEq, Clap)]
enum SubCommand {
    #[clap(name = "start")]
    StartBranch(StartBranchOpts),
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    let current_dir = env::current_dir()?;
    let dir = match git_ex::util::find_git_root_dir(&current_dir) {
        Some(path) => path,
        _ => return Err(anyhow!("Not found git repository.")),
    };
    let mut app = App::new(dir)?;

    match opts.subcmd {
        None => app.start()?,
        Some(s) => match s {
            SubCommand::StartBranch(opts) => {
                let branch = app.start_branch(&opts)?;
                println!("start: {}", branch.name()?.unwrap());
            }
        }
    }
    Ok(())
}
