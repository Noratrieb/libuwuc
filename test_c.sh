#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml"

test_dir=$(mktemp -d)

clean() {
    rm -r "${test_dir}"
}

for test in "$SCRIPT_DIR"/tests/c/*; do
    name=$(basename "$test" .c)
    printf "test %s " "$test"

    flags=$(grep "//@flags: " "$test" | sed 's#//@flags: ##')

    grep "//@ignore" "$test" >/dev/null
    ignore=$?
    if [ "$ignore" -eq "0" ]; then
        echo -e "\e[33mIGNORE\e[0m"
        continue
    fi

    "$SCRIPT_DIR/uwuc-gcc" $flags "$test" -o "$test_dir/$name"

    if [ "$?" -ne "0" ]; then
        echo "error: failed to compile test $test"
        clean
        exit 1
    fi

    cd "$SCRIPT_DIR/tests" || exit 1
    OUTPUT=$("$test_dir/$name")
    code="$?"
    if [ "$code" -ne "0" ]; then
        echo -e "\e[31mFAIL\e[0m"
        echo "error: test failed with code $code: $test, compiled with $flags"
        echo "------output"
        echo -n "$OUTPUT"
        echo "------"
    else
        echo -e "\e[32mPASS\e[0m"
    fi
done

clean
