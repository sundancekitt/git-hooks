# git-hooks
Collection of git hooks. Currently there is only one hook here:

## Prepare commit message hook
This hook automatically adds the branch name to the commit message.
If not present in the commit message already, the current branch name is prepended to the commit message.
On the master branch the commit message is left untouched.

*Note*: This is only useful when the branch name starts with the issue key optionally followed by any string sequence starting with a non-numeric character (BRAHMAN-123 or BRAHMAN-123.2... or BRAHMAN-123-amendements...).

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

