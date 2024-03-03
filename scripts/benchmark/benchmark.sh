#!/usr/bin/env bash

current_time=$(date "+%Y%m%d-%H%M%S")
script_dir=$(dirname "$0")
report="$script_dir/out/report-$current_time.md"
lines_report="$script_dir/out/lines.md"
words_report="$script_dir/out/words.md"
bytes_report="$script_dir/out/bytes.md"
wildcard_report="$script_dir/out/wildcard.md"
warmup=15
runs=100

cargo build --release

local-wc(){
    realpath "./target/release/wc"
}

# hyperfine wrapper
hf() {
    hyperfine --warmup "$warmup" --runs "$runs" "${@}"
}

# expects 2 arg:
# [1] => export file (markdown)
# [2] => mode -w | -l | --words | --lines
# construct args list based on data folder to avoid redundancy
bench-progressive(){
    export_md=$1
    mode=$2

    args=("--export-markdown" "$export_md")
    for data_fn in "$script_dir"/data/*.txt
    do
        data_fn_relative=$(realpath "$data_fn" --relative-to="$script_dir")
        cmd="$(local-wc) $data_fn $mode"
        args+=("-n" "wc $data_fn_relative $mode" "$cmd")
    done

    hf "${args[@]}"

    echo "## \`wc $mode\` benchmark" >> "$report"
    cat "$export_md" >> "$report"
    rm "$export_md"
}

get-data(){
    files=()
    for f in "$script_dir"/data/*.txt
    do
        files+=("$f")
    done

    echo "${files[@]}"
}

bench-wildcard(){
    # evaluate data/*.txt wildcard to get all files
    # data_files=("$script_dir"/data/*.txt)

    wc_path=$(local-wc)

    local_cmd="$wc_path $(get-data)"
    os_cmd="wc $(get-data)"

    hf --export-markdown "$wildcard_report" \
        -n "wc(local) data/*.txt" "$local_cmd" \
        -n "wc(GNU) data/*.txt" "$os_cmd"

    echo "## \`wc data/*.txt\` benchmark"  >> "$report"
    cat "$wildcard_report" >> "$report"
    rm "$wildcard_report"
}


echo "# Benchmark report" > "$report"

bench-wildcard

bench-progressive "$lines_report" --lines
bench-progressive "$words_report" --words
bench-progressive "$bytes_report" --bytes


