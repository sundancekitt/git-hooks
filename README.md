# git-hooks
Collection of git hooks. Currently there is only one hook here:

## Prepare commit message hook
This hook automatically adds the branch name to the commit message for branches that have the naming `feature/<ISSUE_KEY>*`.
If not present in the commit message already, the current branch name is prepended to the commit message.

*Note*: This only applies to branches  that follow the pattern `feature/<ISSUE_KEY>*`, where `<ISSUE_KEY>` is the Jira 
Issue key, for example `CAIRAM-123`. Valid complete branch names can be `feature/CAIRAM-123`, 
`feature-CAIRAM-123-more-details` or `feature/CAIRAM-123.1`.

### Requires
* Rust/cargo

To install `cargo` run the following command:
```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

On Ubuntu you may need to install Openssl C headers:
```
sudo apt install libssl-dev
```

### Build instructions
Run:

```
cargo build --release
```

Copy the binary file under `target/release/prepare-commit-msg` to the desired git hooks directory (under `.git/hooks` in the respective git directory).

Never write the issue key into the commit message again :)

