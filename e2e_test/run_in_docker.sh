#! /bin/bash

set -euxo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

cargo install --path /src/bin
cargo install --path /src/e2e_test/plugin

kyber options /src/e2e_test/examples/main.rs 3:9

kyber do e2e-test-plugin/noop src/main.rs 3:9
