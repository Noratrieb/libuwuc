#!/usr/bin/env bash

# It's like git submodules without the git submodules part.

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

ensure_cloned() {
    if [ ! -d "$SCRIPT_DIR/$2" ]; then
        git clone "$1" "$SCRIPT_DIR/$2"
    fi

    (cd "$SCRIPT_DIR/$2" && git checkout "$3" -q)
}

ensure_cloned "https://github.com/tsoding/noed.git" "tsoding_noed" "1c2bd182139080a8448a59589e8d457a7019d553"
ensure_cloned "https://github.com/zesterer/tosh.git" "zest_tosh" "fa43ee9ec01a625f6a96cb48662f6c8911d8cc8c"
