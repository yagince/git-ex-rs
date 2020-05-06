# git-ex

## Install

### Homebrew (Linux or macOS)

``` shell
$ brew tap yagince/git-ex
$ brew install git-ex
```

### Binaries

[Releases · yagince/git-ex-rs](https://github.com/yagince/git-ex-rs/releases)

## Usage

### Branch Operations

- Delete Branches
- Interactively select a branch to checkout
- View the log for the selected branch.

``` shell
$ git ex
```

### Start topic branch

``` shell
$ git ex start {branch_type} {issue-number} {description}
```

create new branch `{branch_type}/{issue-number}-{description}`
