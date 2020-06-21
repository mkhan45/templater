# templater

Super simple templater for scripts

Usage: `template [file] (var=val)*`

e.g Take loremipsum.txt:
```
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```
Replace "sit" with "stand", "commodo" with "komodo"

```shell
template loremipsum.txt "sit=stand" "commodo=komodo"
```

Output:
```
Lorem ipsum dolor stand amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim ve niam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea komodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```
___

This program doesn't replace everything simultaneously, so it can override previous changes. Because of this, it's recommended to use something to mark you format words. This has the useful side effect of making it faster.

# building

1. Clone the repository, download Rust/Cargo with [rustup](https://rustup.rs/)
2. `cargo build --release`
3. The binary is in the target/release directory
