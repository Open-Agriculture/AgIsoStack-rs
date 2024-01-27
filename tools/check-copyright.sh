#!/bin/bash
set -o errexit
set -o pipefail
set -o nounset
set -o noclobber

REPO_DIRECTORY="$(git rev-parse --show-toplevel)"
RED='\033[0;31m'
GREEN='\033[0;32m'
RESET='\033[0m'

usage() {
    echo "Usage: $0 [--help]"
    echo
    echo "Ensures that each source file has a copyright comment as per the project license"
    echo
    echo "  --help, -h      Show this help and exit"
    echo "  --check, -c     Do not modify the source files, exit with failure if any notices are missing"
}

check_source_file() {
    local source="$1"
    local read_only="$2"
    local failed_check="false"

    if ! head "$source" | grep -i -E "// Copyright 20[0-9]{2} Raven Industries inc" >&/dev/null; then
        failed_check="true"
        if [[ "$read_only" = "false" ]]; then
            local year
            year="$(date +%Y)"
            sed -i "1s;^;// Copyright $year Raven Industries inc\n;" "$source"
        fi
    fi

    if [[ "$failed_check" = "true" ]]; then
        return 1
    fi
}

check_sources() {
    local read_only="$1"
    local failed_check="false"

    shopt -s globstar
    for source in "$REPO_DIRECTORY"/src/**/*.rs; do
        echo -n "Checking '$source' for copyright statement ... "
        if ! check_source_file "$source" "$read_only"; then
            failed_check="true"
            echo -e " ${RED}FAIL${RESET}"
        else
            echo -e " ${GREEN}ok${RESET}"
        fi
    done
    if [[ "$failed_check" = "true" ]]; then
        return 1
    fi
}

main() {
    local read_only="false"

    while [[ $# -gt 0 ]]; do
        case "$1" in
        --help | -h)
            usage
            exit 0
            ;;
        --check | -c)
            read_only="true"
            ;;
        -*)
            echo "Unexpected option: $1" >&2
            exit 1
            ;;
        *)
            echo "Unexpected positional argument: $1" >&2
            exit 1
            ;;
        esac
        shift
    done

    check_sources "$read_only"
}

main "$@"
