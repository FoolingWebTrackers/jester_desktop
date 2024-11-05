# Tauri + Vanilla

This template should help get you started developing with Tauri in vanilla HTML, CSS and Javascript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

#  Entering build env on nix:

run: `nix develop` in the folder where shell.nix exists.

# Starting Dev Server

One time install:
```
cargo install tauri-cli --version '^2.0.0' --locked
```


```
cargo tauri dev
```