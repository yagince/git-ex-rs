use git_ex::app::App;
use std::env;

fn main() -> anyhow::Result<()> {
    let mut app = App::new(env::current_dir()?)?;
    app.start()?;
    Ok(())
}
