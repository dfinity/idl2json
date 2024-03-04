# Hacking

## To create a release
- Update the version number:
  - in `Cargo.toml`
  - in `src/*_cli/Cargo.toml` where the version number appears in the dependencies
  - in `Cargo.lock` by running `cargo fetch`.
- Create a PR & merge into main.
  - Note: For a pre-release version you MAY release from a branch.  In that case there is no need to merge to main but you should:
    - Set the version to `x.y.z-alpha.A` e.g. `1.2.3-alpha.1`.
    - Wait for CI to be green
- Run this GitHub workflow by clicking the button on the right: https://github.com/dfinity/idl2json/actions/workflows/publish.yml
