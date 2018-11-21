#! /bin/bash

set -euxo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

TARGET_DIR="$DIR/../target/e2e"

rm -rf $TARGET_DIR
