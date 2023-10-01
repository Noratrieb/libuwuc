#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml"

test_dir=$(mktemp -d)

clean() {
    rm -r "${test_dir}"
}

for test in tests/c/*; do
    name=$(basename $test .c)

    "$SCRIPT_DIR/uwuc-gcc" "$test" -o "$test_dir/$name"

    if [ "$?" -ne "0" ]; then
        echo "error: failed to compile test $test"
        clean
        exit 1
    fi

    printf "test $name "

    OUTPUT=$("$test_dir/$name")
    code="$?"
    if [ "$code" -ne "0" ]; then
        echo -e "\e[31mFAIL\e[0m"
        echo "error: test failed with code $code: $test, running $test_dir/$name"
        echo "------ output:"
        echo "$OUTPUT"
        echo "-----"
    else
        echo -e "\e[32mPASS\e[0m"
    fi
done

clean
