use clap::Clap;

#[derive(Debug, Clone, PartialEq, Clap)]
pub struct StartBranchOpts {
    /// f: feature, h: hotfix, s: spark, etc...
    branch_type: BranchType,
    /// Issue number.
    issue_number: String,
    /// description of branch.
    description: String,
}

impl StartBranchOpts {
    pub fn branch_name(&self) -> String {
        format!(
            "{}/{}-{}",
            self.branch_type, self.issue_number, self.description
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BranchType {
    Feature,
    Hotfix,
    Spark,
    Other(String),
}

impl std::str::FromStr for BranchType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "f" | "feature" | "Feature" => BranchType::Feature,
            "h" | "hotfix" | "Hotfix" => BranchType::Hotfix,
            "s" | "spark" | "Spark" => BranchType::Spark,
            _ => BranchType::Other(s.to_lowercase()),
        })
    }
}

impl std::fmt::Display for BranchType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BranchType::Other(s) => {
                write!(f, "{}", s)
            },
            BranchType::Feature => {
                write!(f, "feature")
            },
            BranchType::Hotfix => {
                write!(f, "hotfix")
            },
            BranchType::Spark => {
                write!(f, "spark")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_branch_type_from_str() -> anyhow::Result<()> {
        assert_eq!(BranchType::from_str("f")?, BranchType::Feature);
        assert_eq!(BranchType::from_str("feature")?, BranchType::Feature);
        assert_eq!(BranchType::from_str("Feature")?, BranchType::Feature);

        assert_eq!(BranchType::from_str("h")?, BranchType::Hotfix);
        assert_eq!(BranchType::from_str("hotfix")?, BranchType::Hotfix);
        assert_eq!(BranchType::from_str("Hotfix")?, BranchType::Hotfix);

        assert_eq!(BranchType::from_str("s")?, BranchType::Spark);
        assert_eq!(BranchType::from_str("spark")?, BranchType::Spark);
        assert_eq!(BranchType::from_str("Spark")?, BranchType::Spark);

        assert_eq!(
            BranchType::from_str("hoge")?,
            BranchType::Other("hoge".into())
        );
        assert_eq!(
            BranchType::from_str("Hoge")?,
            BranchType::Other("hoge".into())
        );
        Ok(())
    }

    #[test]
    fn test_branch_opts_to_branch_name() -> anyhow::Result<()> {
        let opts = StartBranchOpts {
            branch_type: BranchType::Feature,
            issue_number: "number".into(),
            description: "test-desc".into(),
        };

        assert_eq!(opts.branch_name(), "feature/number-test-desc");

        let opts = StartBranchOpts {
            branch_type: BranchType::Hotfix,
            issue_number: "number".into(),
            description: "test-desc".into(),
        };

        assert_eq!(opts.branch_name(), "hotfix/number-test-desc");

        let opts = StartBranchOpts {
            branch_type: BranchType::Spark,
            issue_number: "number".into(),
            description: "test-desc".into(),
        };

        assert_eq!(opts.branch_name(), "spark/number-test-desc");

        let opts = StartBranchOpts {
            branch_type: BranchType::Other("hoge".into()),
            issue_number: "number".into(),
            description: "test-desc".into(),
        };

        assert_eq!(opts.branch_name(), "hoge/number-test-desc");
        Ok(())
    }
}
