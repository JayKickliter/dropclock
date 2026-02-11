# Releasing

## In a PR

1. Bump `version` in `Cargo.toml`
2. Add an entry to `CHANGELOG.md`

## After merge to `main`

1. `git pull`
2. `git tag -a vx.y.z -m "vx.y.z"`
3. `cargo publish --dry-run` — make sure it looks right
4. `cargo publish`
5. `git push --tags`
