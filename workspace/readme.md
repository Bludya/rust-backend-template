# Workspace

# How to use

1. Install "Remote development" extension.
2. Copy everything from <code>/parentDir/rust-template/workspace</code> to <code>parentDir</code>
3. Rename/copy <code>parentDir/.devcontainer/.env.example</code> to <code>parentDir/.devcontainer/.env</code>. There are 7 parameters that you must specify:

        3.1. MNT_PATH - This is the exact path there <code>parentDir</code> is located in the host. Open it in shell and run <code>pwd</code>. The result of the command is the value of MNT_PATH.
        3.2. PASS - This is the password which will be used inside dev container for both root and your accounts.
        3.3. USER_ID - Id of your host's account
        3.4. USER_NAME - Name of your host's account
        3.5. GROUP_ID - Id of your host's account's group
        3.6. GROUP_NAME - Name of your host's account's group
        3.7. DOCKER_GROUP_ID - Id of docker's group from the host

4. Open rust.code-workspace located in <code>parentDir</code>
5. Open View -> Command pallete and type "Reopen in container".

Now your VS Code should reopen and start building the devcontainer. It could take some time. Once it is ready you will see in bottom-left corner the following label "Dev Container: rust-workspace". In this devcontainer you have already installed Rust, essential Rust cargo plugins, Docker, Docker compose.

# Rust Cargo plugins installed

`cargo edit` - easy cmd line tool to add/upgrade/remove cargo crates
`cargo watch` - checks for code changes an runs command on one. So for example it could `cargo run` on saving file. Example use: `cargo watch -c -w -x run`
`cargo release` - help with crate releases. More info here: https://github.com/crate-ci/cargo-release
`cargo make` - create build scripts. Check build.toml for more info on use, as well as example command : `cargo make --makefile build.toml build-flow`
`cargo nextest` - next gen Rust test runner. Prettier output and more importantly faster test runs. Use: `cargo nextest run`.
`cargo audit` - checks for dependancy issues.
`cargo tarpaulin` - check test coverage.
`cargo modules` - see module structure in a tree format. Example: `cargo modules generate tree`
