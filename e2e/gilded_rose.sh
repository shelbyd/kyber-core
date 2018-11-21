#! /bin/bash

set -euo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

PROJECT_DIR="$DIR/.."
TARGET_DIR="$DIR/../target/e2e/gilded_rose"

function clone_git_repo() {
  if [ -f Cargo.toml ]; then
    echo "already cloned gilded rose"
  else
    git clone https://github.com/saterus/gilded-rose-rust .
  fi
  reset_git_repo
}

function reset_git_repo() {
  git reset --hard 8a8ec0b54b71eb6cfc764dda770b60843d95b73e
  git clean -fd .
}

function apply_basic_fixes() {
  sed -i "s/UpdateQuality/update_quality/" src/main.rs

  cargo install cargo-fix --quiet || true
  cargo fix --allow-no-vcs --quiet

  cargo install rustfmt --quiet || true
  cargo fmt
}

function kyber_commands() {
  kyber "do" extract_variable src/main.rs 53,12:53,19
}

function kyber() {
  cargo run --manifest-path $PROJECT_DIR/Cargo.toml -- $@
}

mkdir -p $TARGET_DIR
cd $TARGET_DIR

clone_git_repo
apply_basic_fixes

kyber_commands
