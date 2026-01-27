# justfile
# `just build[-all]` -> dev, or `just build[-all] profile=release` -> release

profile := "debug"

[private]
default: list

# List all recipes.
list:
    just --list

# Build with `just build` -> dev, or `just build profile=release` -> release
build:
    clear
    cd macros && cargo build {{ if profile == "release" { "--release" } else { "" } }}
    cargo build {{ if profile == "release" { "--release" } else { "" } }}

# Build with `just build-all` -> dev, or `just build-all profile=release` -> release
build-all: cross-linux cross-windows cross-macos-x64 cross-macos-aarch64

# Clear cache, delete temps, and I think it also deletes the built files.
clean:
    cd macros && cargo clean
    cargo clean

# Reformat the code as defined in the style guide.
fmt:
    cd macros && cargo fmt
    cargo fmt

# Add all unignored files to git and make a commit with MESSAGE as the commit message.
commit *MESSAGE="Unlabeled commit.": ci
    git add .
    git commit -m "{{ MESSAGE }}"

# Push commits to remote repository.
push:
    git push -u origin main

# Pull commits from remote repository.
pull:
    git pull

# Run `cargo check`, `cargo fmt --check`, `cargo clippy -- -D warnings` and same with `-W clippy::pedantic`, `cargo test`, and `cargo build`.
ci: ci-macros-only
    cargo check
    cargo check --features=parse
    cargo check --features=bevy
    cargo check --features=egui
    cargo check --features=bevy,egui
    cargo check --features=parse,bevy,egui
    cargo fmt --check
    cargo clippy -- -D warnings
    cargo clippy -- -D warnings -W clippy::pedantic
    cargo clippy --features=parse -- -D warnings
    cargo clippy --features=parse -- -D warnings -W clippy::pedantic
    cargo clippy --features=bevy -- -D warnings
    cargo clippy --features=bevy -- -D warnings -W clippy::pedantic
    cargo clippy --features=egui -- -D warnings
    cargo clippy --features=egui -- -D warnings -W clippy::pedantic
    cargo clippy --features=bevy,egui -- -D warnings
    cargo clippy --features=bevy,egui -- -D warnings -W clippy::pedantic
    cargo clippy --features=parse,bevy,egui -- -D warnings
    cargo clippy --features=parse,bevy,egui -- -D warnings -W clippy::pedantic
    cargo test
    cargo test --features=parse
    cargo test --features=bevy
    cargo test --features=egui
    cargo test --features=bevy,egui
    cargo test --features=parse,bevy,egui
    cargo build
    cargo build --features=parse
    cargo build --features=bevy
    cargo build --features=egui
    cargo build --features=bevy,egui
    cargo build --features=parse,bevy,egui

# Run `cargo check`, `cargo fmt --check`, `cargo clippy -- -D warnings` and same with `-W clippy::pedantic`, `cargo test`, and `cargo build`.
ci-macros-only:
    clear
    cd macros && cargo check
    cd macros && cargo fmt --check
    cd macros && cargo clippy -- -D warnings
    cd macros && cargo clippy -- -D warnings -W clippy::pedantic
    cd macros && cargo test
    cd macros && cargo build

publish-dry-run-macros: ci-macros-only
    cargo publish -p bevy-color-macros --dry-run

publish-dry-run-palettes: ci
    cargo publish -p bevy-color-palettes --dry-run

publish-for-real-macros: ci-macros-only
    cargo publish -p bevy-color-macros --dry-run
    cargo publish -p bevy-color-macros

publish-for-real-palettes: ci
    cargo publish -p bevy-color-palettes --dry-run
    cargo publish -p bevy-color-palettes

# Added target with `rustup target add x86_64-unknown-linux-gnu`
[private]
cross-linux:
    cd macros && cargo build --target x86_64-unknown-linux-gnu {{ if profile == "release" { "--release" } else { "" } }}
    cargo build --target x86_64-unknown-linux-gnu {{ if profile == "release" { "--release" } else { "" } }}

# Added target with `rustup target add x86_64-pc-windows-gnu`
[private]
cross-windows:
    cd macros && cargo build --target x86_64-pc-windows-gnu {{ if profile == "release" { "--release" } else { "" } }}
    cargo build --target x86_64-pc-windows-gnu {{ if profile == "release" { "--release" } else { "" } }}

# Added target with `rustup target add x86_64-apple-darwin`
[private]
cross-macos-x64:
    cd macros && cargo build --target x86_64-apple-darwin {{ if profile == "release" { "--release" } else { "" } }}
    cargo build --target x86_64-apple-darwin {{ if profile == "release" { "--release" } else { "" } }}

# Added target with `rustup target add aarch64-apple-darwin`
[private]
cross-macos-aarch64:
    cd macros && cargo build --target aarch64-apple-darwin {{ if profile == "release" { "--release" } else { "" } }}
    cargo build --target aarch64-apple-darwin {{ if profile == "release" { "--release" } else { "" } }}
