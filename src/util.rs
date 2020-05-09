use std::path::{Path, PathBuf};

pub mod event;
pub mod stateful_list;

pub use event::{Event, Events};
pub use stateful_list::StatefulList;

use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn centered_fix_rect(width: u16, height: u16, r: Rect) -> Rect {
    let width = std::cmp::min(r.width, width);
    let height = std::cmp::min(r.height, height);

    Rect::new(
        (r.width - width) / 2,
        (r.height - height) / 2,
        width,
        height,
    )
}

pub fn to_refs(name: &str) -> String {
    format!("refs/heads/{}", name)
}

pub fn find_git_root_dir<P: AsRef<Path>>(path: &P) -> Option<PathBuf> {
    if !path.as_ref().is_dir() {
        return None;
    }
    if path
        .as_ref()
        .read_dir()
        .unwrap()
        .any(|r| r.map(|x| x.path().ends_with(".git")).unwrap_or(false))
    {
        return Some(path.as_ref().to_owned());
    }

    match path.as_ref().parent() {
        Some(ref path) => find_git_root_dir(path),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_refs() {
        assert_eq!(to_refs("test"), "refs/heads/test")
    }

    #[test]
    fn test_find_git_root_dir() -> anyhow::Result<()> {
        let dir = std::env::current_dir()?;
        assert_eq!(find_git_root_dir(&dir), Some(dir));

        let dir = std::env::current_dir()?;
        let file = dir.with_file_name("hoge.txt");
        assert_eq!(find_git_root_dir(&file), None);

        let dir = std::env::current_dir()?;
        let file = dir.join("src");
        assert_eq!(find_git_root_dir(&file), Some(dir));

        Ok(())
    }
}
