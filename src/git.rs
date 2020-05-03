use std::path::Path;

use chrono::TimeZone;

pub struct Signature {
    pub name: String,
    pub email: String,
}

pub struct Commit {
    pub id: String,
    pub author: Signature,
    pub message: String,
    pub datetime: chrono::DateTime<chrono::Local>,
}

pub struct Repository {
    repo: git2::Repository,
}

impl Repository {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        Ok(Self {
            repo: git2::Repository::open(path)?,
        })
    }

    pub fn logs(&self, branch_name: &str, limit: usize) -> anyhow::Result<Vec<Commit>> {
        let branch = self
            .repo
            .find_branch(branch_name, git2::BranchType::Local)?;
        let mut revwalk = self.repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::NONE | git2::Sort::TIME)?;
        revwalk.push_ref(branch.get().name().unwrap())?;

        Ok(revwalk
            .flat_map(|id| self.repo.find_commit(id.unwrap()))
            .map(|commit| Commit {
                id: commit
                    .as_object()
                    .short_id()
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .into(),
                author: Signature {
                    name: String::from_utf8_lossy(commit.author().name_bytes()).into(),
                    email: String::from_utf8_lossy(commit.author().email_bytes()).into(),
                },
                message: String::from_utf8_lossy(commit.message_bytes()).into(),
                datetime: chrono::Local.timestamp(commit.time().seconds(), 0),
            })
            .take(limit)
            .collect())
    }
}
