use std::path::Path;

pub struct Log {
    pub id: String,
    pub author: String,
    pub message: String,
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

    pub fn logs(&self, branch_name: &str, limit: usize) -> anyhow::Result<Vec<Log>> {
        let branch = self
            .repo
            .find_branch(branch_name, git2::BranchType::Local)?;
        let mut revwalk = self.repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::NONE | git2::Sort::TIME)?;
        revwalk.push_ref(branch.get().name().unwrap())?;

        Ok(revwalk
            .flat_map(|id| self.repo.find_commit(id.unwrap()))
            .map(|commit| Log {
                id: commit
                    .as_object()
                    .short_id()
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .into(),
                author: String::default(),
                message: String::from_utf8_lossy(commit.message_bytes()).into(),
            })
            .take(limit)
            .collect())
    }
}
