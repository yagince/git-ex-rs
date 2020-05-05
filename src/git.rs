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

    pub fn current_branch(&self) -> anyhow::Result<Option<String>> {
        self.repo
            .head()
            .and_then(|reference| Ok(reference.shorthand().map(ToOwned::to_owned)))
            .map_err(Into::into)
    }

    pub fn branches(&self) -> anyhow::Result<Vec<String>> {
        self.repo
            .branches(Some(git2::BranchType::Local))?
            .map(|x| {
                let (branch, _) = x?;
                Ok(branch.name()?.unwrap().to_owned())
            })
            .collect()
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

    pub fn checkout(&self, branch_name: &str) -> anyhow::Result<()> {
        self.repo
            .find_branch(branch_name, git2::BranchType::Local)
            .and_then(|branch| self.repo.set_head(branch.get().name().unwrap()))
            .map_err(Into::into)
    }

    pub fn delete_branch(&self, branch_name: &str) -> anyhow::Result<()> {
        self.repo
            .find_branch(branch_name, git2::BranchType::Local)
            .and_then(|mut b| b.delete())
            .map_err(Into::into)
    }
}
