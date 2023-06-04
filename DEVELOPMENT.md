# Development

## Dependencies

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
cargo install cargo-nextest
```


## Running Tests

```bash
cargo nextest run --workspace --all-features

# To test individual features
for i in {0..0}; do cargo test_$i || break; done
```


## Coverage

Collect coverage and output as `lcov`.

```bash
./coverage.sh
```

Collect coverage and open `html` report.

```bash
./coverage.sh && cargo coverage_open
```


## Releasing

1. Update crate versions.

    ```bash
    sd -s 'version = "0.8.0"' 'version = "0.9.0"' $(fd -tf -F toml) README.md src/lib.rs

    # Make sure only `proc_macro_roids` crates are updated.
    git --no-pager diff | rg '^[+]' | rg -v '(proc_macro_roids)|(\+\+\+)|\+version'
    ```

2. Update `CHANGELOG.md` with the version and today's date.
3. Push a tag to the repository.

    The [`publish`] GitHub workflow will automatically publish the crates to [`crates.io`].

[`publish`]: https://github.com/azriel91/proc_macro_roids/actions/workflows/publish.yml
[`crates.io`]:https://crates.io/

**Note:**

An alternative to `cargo-release` is [`cargo-workspaces`], which may be used in case crates need to be published one by one -- if many new crates are being published, `cargo-release` gates the number of crates that can be published at one go.

```bash
cargo workspaces \
  publish \
  --from-git \
  --allow-branch main \
  --force '*' \
  --no-git-tag
```

[`cargo-workspaces`]: https://github.com/pksunkara/cargo-workspaces
