#! /bin/bash

set -euxo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

docker build $DIR -t kyber-e2e-test

docker run -v $(dirname $DIR):/src -t kyber-e2e-test bash /src/e2e_test/run_in_docker.sh
