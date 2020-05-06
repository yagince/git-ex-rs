use git_ex::{app::App, cmd::StartBranchOpts};
use std::env;

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

    let mut app = App::new(env::current_dir()?)?;

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
