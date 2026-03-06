Simple test for DataTypes
Things I learned:

- UpperCamelCase for things like types, and lower_snake_case for things like variables and functions. (https://www.reddit.com/r/rust/comments/w2nxrw/snake_case_vs_camel_case/)
- VSCode's Rust analyzer doesn't work on nested Cargo.toml files by default and they need to be listed in the extension's settings (not recommanded: https://users.rust-lang.org/t/in-vscode-rust-analyzer-features-such-as-code-completion-doesnt-work-in-certain-conditions/107685/6). However, the extension is nice and all when it works.
- setting for VSCode:
```
  "rust-analyzer.check.command": "clippy",
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true,
  },
```
- Comment blocks seem to be prefered as ```/// comment```
- See inline comments in main.rs

Commands I learned / used:
- `rustup component add rustfmt --toolchain nightly`
- `rustfmt src/main.rs`
- `cargo run` => will show some warnings such as unused variables
